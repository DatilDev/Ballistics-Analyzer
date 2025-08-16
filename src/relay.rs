use std::{net::SocketAddr, sync::Arc};
use chrono::Utc;
use futures::{SinkExt, StreamExt};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

#[derive(Clone)]
pub struct LocalRelay {
    db: Arc<Mutex<Connection>>,
    bind: SocketAddr,
}

#[derive(Debug, Serialize)]
struct RelayInfo {
    name: String,
    description: String,
    pubkey: Option<String>,
    software: String,
    version: String,
}

impl LocalRelay {
    pub fn new(bind: SocketAddr, db_path: &str) -> anyhow::Result<Self> {
        let conn = Connection::open(db_path)?;
        init_db(&conn)?;
        Ok(Self {
            db: Arc::new(Mutex::new(conn)),
            bind,
        })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(self.bind).await?;
        tracing::info!("Local Nostr relay listening on ws://{}", self.bind);

        loop {
            let (stream, addr) = listener.accept().await?;
            let peer = format!("{}", addr);
            let relay = self.clone();

            tokio::spawn(async move {
                match tokio_tungstenite::accept_async(stream).await {
                    Ok(ws) => {
                        if let Err(e) = relay.handle_connection(ws, &peer).await {
                            tracing::warn!("Connection {} error: {:?}", peer, e);
                        }
                    }
                    Err(e) => tracing::warn!("WS handshake err from {}: {:?}", peer, e),
                }
            });
        }
    }

    async fn handle_connection(
        &self,
        mut ws: WebSocketStream<tokio::net::TcpStream>,
        _peer: &str,
    ) -> anyhow::Result<()> {
        while let Some(msg) = ws.next().await {
            let msg = msg?;
            if msg.is_text() {
                let txt = msg.into_text()?;
                if txt == "GET /" || txt == "GET / HTTP/1.1" {
                    // NIP-11 if someone pokes via plain HTTP-style text
                    let info = RelayInfo {
                        name: "Local Personal Relay".to_string(),
                        description: "Local relay for Ballistics Analyzer".to_string(),
                        pubkey: None,
                        software: "ballistics-analyzer-local-relay".to_string(),
                        version: "0.1.0".to_string(),
                    };
                    let _ = ws
                        .send(Message::text(serde_json::to_string(&info)?))
                        .await;
                    continue;
                }

                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&txt) {
                    if let Some(arr) = v.as_array() {
                        if arr.is_empty() {
                            continue;
                        }
                        let cmd = arr[0].as_str().unwrap_or_default();
                        match cmd {
                            "EVENT" => {
                                if arr.len() >= 2 {
                                    if let Some(event) = arr[1].as_object() {
                                        if let Err(e) = self.store_event(event).await {
                                            tracing::warn!("store_event error: {:?}", e);
                                            let _ = ws
                                                .send(Message::text(
                                                    r#"["OK", "", false, "error"]"#,
                                                ))
                                                .await;
                                        } else {
                                            let id = event
                                                .get("id")
                                                .and_then(|x| x.as_str())
                                                .unwrap_or("");
                                            let _ = ws
                                                .send(Message::text(format!(
                                                    r#"["OK","{id}",true,""]"#
                                                )))
                                                .await;
                                        }
                                    }
                                }
                            }
                            "REQ" => {
                                // ["REQ", <subid>, {filters...}, {filters...}]
                                if arr.len() >= 2 {
                                    let subid = arr[1].as_str().unwrap_or("sub");
                                    let filters = arr[2..].to_vec();
                                    let events = self.query_events(filters).await.unwrap_or_default();
                                    for ev in events {
                                        let _ = ws
                                            .send(Message::text(format!(
                                                r#"[ "EVENT", "{}", {} ]"#,
                                                subid, ev
                                            )))
                                            .await;
                                    }
                                    let _ = ws
                                        .send(Message::text(format!(
                                            r#"[ "EOSE", "{}" ]"#,
                                            subid
                                        )))
                                        .await;
                                }
                            }
                            "CLOSE" => {
                                // ["CLOSE", <subid>]
                            }
                            _ => {}
                        }
                    }
                }
            } else if msg.is_close() {
                break;
            }
        }
        Ok(())
    }

    async fn store_event(&self, event: &serde_json::Map<String, serde_json::Value>) -> anyhow::Result<()> {
        let id = event.get("id").and_then(|x| x.as_str()).unwrap_or_default();
        let pubkey = event.get("pubkey").and_then(|x| x.as_str()).unwrap_or_default();
        let kind = event.get("kind").and_then(|x| x.as_i64()).unwrap_or(0i64) as i32;
        let created_at = event.get("created_at").and_then(|x| x.as_i64()).unwrap_or(Utc::now().timestamp());
        let content = event.get("content").and_then(|x| x.as_str()).unwrap_or("");
        let tags = event.get("tags").cloned().unwrap_or_else(|| serde_json::json!([]));

        let sql = r#"
            INSERT OR IGNORE INTO events (id, pubkey, kind, created_at, content, tags)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#;

        let db = self.db.lock().await;
        db.execute(
            sql,
            params![id, pubkey, kind, created_at, content, serde_json::to_string(&tags)?],
        )?;

        Ok(())
    }

    async fn query_events(&self, filters: Vec<serde_json::Value>) -> anyhow::Result<Vec<String>> {
        // Minimal filter: ids, authors, kinds, since, until
        let mut out = Vec::new();
        let db = self.db.lock().await;

        for f in filters {
            let mut sql = String::from("SELECT id, pubkey, kind, created_at, content, tags FROM events WHERE 1=1");
            let mut args: Vec<(usize, String)> = Vec::new();

            if let Some(ids) = f.get("ids").and_then(|x| x.as_array()) {
                if !ids.is_empty() {
                    sql.push_str(" AND id IN (");
                    for (i, _) in ids.iter().enumerate() {
                        if i > 0 { sql.push(','); }
                        sql.push('?');
                    }
                    sql.push(')');
                    for (i, idv) in ids.iter().enumerate() {
                        if let Some(s) = idv.as_str() {
                            args.push((i + 1, s.to_string()));
                        }
                    }
                }
            }

            if let Some(authors) = f.get("authors").and_then(|x| x.as_array()) {
                for a in authors {
                    if let Some(s) = a.as_str() {
                        sql.push_str(" AND pubkey = ?");
                        args.push((args.len() + 1, s.to_string()));
                    }
                }
            }

            if let Some(kinds) = f.get("kinds").and_then(|x| x.as_array()) {
                if !kinds.is_empty() {
                    sql.push_str(" AND kind IN (");
                    for (i, _) in kinds.iter().enumerate() {
                        if i > 0 { sql.push(','); }
                        sql.push('?');
                    }
                    sql.push(')');
                    for k in kinds {
                        if let Some(n) = k.as_i64() {
                            args.push((args.len() + 1, n.to_string()));
                        }
                    }
                }
            }

            if let Some(since) = f.get("since").and_then(|x| x.as_i64()) {
                sql.push_str(" AND created_at >= ?");
                args.push((args.len() + 1, since.to_string()));
            }
            if let Some(until) = f.get("until").and_then(|x| x.as_i64()) {
                sql.push_str(" AND created_at <= ?");
                args.push((args.len() + 1, until.to_string()));
            }

            sql.push_str(" ORDER BY created_at DESC LIMIT 1000");

            let mut stmt = db.prepare(&sql)?;
            let rows = stmt.query_map(
                rusqlite::params_from_iter(args.iter().map(|(_, v)| v)),
                |row| {
                    let id: String = row.get(0)?;
                    let pubkey: String = row.get(1)?;
                    let kind: i32 = row.get(2)?;
                    let created_at: i64 = row.get(3)?;
                    let content: String = row.get(4)?;
                    let tags_json: String = row.get(5)?;
                    let tags: serde_json::Value = serde_json::from_str(&tags_json).unwrap_or_else(|_| serde_json::json!([]));

                    let ev = serde_json::json!({
                        "id": id,
                        "pubkey": pubkey,
                        "kind": kind,
                        "created_at": created_at,
                        "content": content,
                        "tags": tags,
                        "sig": "" // signature is part of event published by client; we store as-is
                    });

                    Ok(serde_json::to_string(&ev).unwrap())
                },
            )?;

            for r in rows {
                if let Ok(ev) = r {
                    out.push(ev);
                }
            }
        }

        Ok(out)
    }
}

fn init_db(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        r#"
        PRAGMA journal_mode = WAL;
        CREATE TABLE IF NOT EXISTS events (
            id TEXT PRIMARY KEY,
            pubkey TEXT NOT NULL,
            kind INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            content TEXT NOT NULL,
            tags TEXT NOT NULL
        );
        "#,
    )?;
    Ok(())
}