-- Create manufacturers table
CREATE TABLE IF NOT EXISTS manufacturers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

-- Create load_data table
CREATE TABLE IF NOT EXISTS load_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    manufacturer_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    caliber TEXT NOT NULL,
    bullet_weight REAL NOT NULL,
    velocity REAL NOT NULL,
    bc REAL NOT NULL,
    powder_type TEXT,
    powder_charge REAL,
    category TEXT NOT NULL DEFAULT 'Rifle', -- 'Rifle', 'Pistol', 'Rimfire'
    FOREIGN KEY (manufacturer_id) REFERENCES manufacturers(id),
    UNIQUE(manufacturer_id, name)
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_load_data_manufacturer ON load_data(manufacturer_id);
CREATE INDEX IF NOT EXISTS idx_load_data_caliber ON load_data(caliber);
CREATE INDEX IF NOT EXISTS idx_load_data_category ON load_data(category);

-- Insert manufacturers
INSERT OR IGNORE INTO manufacturers (name) VALUES 
    ('Federal'),
    ('Hornady'),
    ('CCI'),
    ('Aguila'),
    ('Remington');

-- Insert Federal ammunition data
INSERT OR IGNORE INTO load_data (manufacturer_id, name, caliber, bullet_weight, velocity, bc, powder_type, powder_charge, category) VALUES
    -- Federal Rifle
    ((SELECT id FROM manufacturers WHERE name='Federal'), 'Gold Medal Match 308 Win 175gr', '.308 Winchester', 175.0, 2600.0, 0.505, 'IMR 4064', 42.5, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Federal'), 'Gold Medal Match 308 Win 168gr', '.308 Winchester', 168.0, 2650.0, 0.462, 'IMR 4064', 43.5, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Federal'), 'Premium 6.5 Creedmoor 140gr', '6.5 Creedmoor', 140.0, 2750.0, 0.610, 'H4350', 41.5, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Federal'), 'Premium 300 Win Mag 180gr', '.300 Winchester Magnum', 180.0, 2960.0, 0.507, 'H1000', 74.0, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Federal'), 'Premium 223 Rem 69gr', '.223 Remington', 69.0, 3000.0, 0.301, 'Varget', 24.5, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Federal'), 'Premium 270 Win 130gr', '.270 Winchester', 130.0, 3060.0, 0.436, 'H4831', 58.0, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Federal'), 'Premium 30-06 165gr', '.30-06 Springfield', 165.0, 2800.0, 0.477, 'IMR 4350', 56.0, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Federal'), 'Premium 7mm Rem Mag 160gr', '7mm Remington Magnum', 160.0, 2950.0, 0.531, 'RL22', 68.0, 'Rifle'),
    -- Federal Pistol
    ((SELECT id FROM manufacturers WHERE name='Federal'), 'HST 9mm 124gr +P', '9mm Luger', 124.0, 1200.0, 0.165, 'Power Pistol', 6.5, 'Pistol'),
    ((SELECT id FROM manufacturers WHERE name='Federal'), 'HST 45 ACP 230gr', '.45 ACP', 230.0, 890.0, 0.195, 'Unique', 6.0, 'Pistol'),
    ((SELECT id FROM manufacturers WHERE name='Federal'), 'HST 40 S&W 180gr', '.40 S&W', 180.0, 1010.0, 0.164, 'Power Pistol', 7.5, 'Pistol');

-- Insert Hornady ammunition data
INSERT OR IGNORE INTO load_data (manufacturer_id, name, caliber, bullet_weight, velocity, bc, powder_type, powder_charge, category) VALUES
    -- Hornady Rifle
    ((SELECT id FROM manufacturers WHERE name='Hornady'), 'Match 6.5 Creedmoor 147gr ELD-M', '6.5 Creedmoor', 147.0, 2695.0, 0.697, 'H4350', 40.8, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Hornady'), 'Precision Hunter 300 Win Mag 200gr', '.300 Winchester Magnum', 200.0, 2850.0, 0.597, 'H1000', 72.0, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Hornady'), 'Match 308 Win 168gr ELD-M', '.308 Winchester', 168.0, 2700.0, 0.523, 'Varget', 44.0, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Hornady'), 'Superformance 223 Rem 75gr', '.223 Remington', 75.0, 2930.0, 0.395, 'Superformance', 25.5, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Hornady'), 'Precision Hunter 6.5 PRC 143gr', '6.5 PRC', 143.0, 2960.0, 0.625, 'RL26', 56.5, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Hornady'), 'Match 6mm Creedmoor 108gr', '6mm Creedmoor', 108.0, 2960.0, 0.536, 'H4350', 40.0, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Hornady'), 'Precision Hunter 270 Win 145gr', '.270 Winchester', 145.0, 2970.0, 0.536, 'H4831SC', 56.0, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Hornady'), 'Match 338 Lapua 285gr', '.338 Lapua Magnum', 285.0, 2745.0, 0.789, 'Retumbo', 89.0, 'Rifle'),
    -- Hornady Pistol
    ((SELECT id FROM manufacturers WHERE name='Hornady'), 'Critical Defense 9mm 115gr', '9mm Luger', 115.0, 1135.0, 0.157, 'Unique', 5.8, 'Pistol'),
    ((SELECT id FROM manufacturers WHERE name='Hornady'), 'Critical Duty 45 ACP +P 220gr', '.45 ACP', 220.0, 975.0, 0.188, 'Longshot', 7.2, 'Pistol'),
    ((SELECT id FROM manufacturers WHERE name='Hornady'), 'Critical Defense 380 ACP 90gr', '.380 ACP', 90.0, 1000.0, 0.128, 'HP-38', 3.8, 'Pistol');

-- Insert CCI ammunition data
INSERT OR IGNORE INTO load_data (manufacturer_id, name, caliber, bullet_weight, velocity, bc, powder_type, powder_charge, category) VALUES
    -- CCI Rimfire
    ((SELECT id FROM manufacturers WHERE name='CCI'), 'Standard Velocity 22 LR 40gr', '.22 Long Rifle', 40.0, 1070.0, 0.138, 'Rimfire Powder', 1.5, 'Rimfire'),
    ((SELECT id FROM manufacturers WHERE name='CCI'), 'Mini-Mag 22 LR 36gr HP', '.22 Long Rifle', 36.0, 1260.0, 0.125, 'Rimfire Powder', 1.8, 'Rimfire'),
    ((SELECT id FROM manufacturers WHERE name='CCI'), 'Stinger 22 LR 32gr HP', '.22 Long Rifle', 32.0, 1640.0, 0.118, 'Rimfire Powder', 2.0, 'Rimfire'),
    ((SELECT id FROM manufacturers WHERE name='CCI'), 'Velocitor 22 LR 40gr HP', '.22 Long Rifle', 40.0, 1435.0, 0.125, 'Rimfire Powder', 2.2, 'Rimfire'),
    ((SELECT id FROM manufacturers WHERE name='CCI'), '22 WMR Maxi-Mag 40gr', '.22 Winchester Magnum', 40.0, 1875.0, 0.133, 'Rimfire Powder', 3.5, 'Rimfire'),
    ((SELECT id FROM manufacturers WHERE name='CCI'), '17 HMR 17gr V-Max', '.17 HMR', 17.0, 2550.0, 0.125, 'Rimfire Powder', 3.0, 'Rimfire'),
    -- CCI Pistol
    ((SELECT id FROM manufacturers WHERE name='CCI'), 'Blazer Brass 9mm 115gr FMJ', '9mm Luger', 115.0, 1145.0, 0.155, 'Titegroup', 4.8, 'Pistol'),
    ((SELECT id FROM manufacturers WHERE name='CCI'), 'Blazer Brass 45 ACP 230gr FMJ', '.45 ACP', 230.0, 830.0, 0.195, 'HP-38', 5.5, 'Pistol'),
    ((SELECT id FROM manufacturers WHERE name='CCI'), 'Blazer Brass 40 S&W 180gr FMJ', '.40 S&W', 180.0, 1000.0, 0.164, 'Universal', 6.5, 'Pistol');

-- Insert Aguila ammunition data
INSERT OR IGNORE INTO load_data (manufacturer_id, name, caliber, bullet_weight, velocity, bc, powder_type, powder_charge, category) VALUES
    -- Aguila Rimfire
    ((SELECT id FROM manufacturers WHERE name='Aguila'), 'Super Extra 22 LR 40gr', '.22 Long Rifle', 40.0, 1255.0, 0.138, 'Rimfire Powder', 1.7, 'Rimfire'),
    ((SELECT id FROM manufacturers WHERE name='Aguila'), 'Interceptor 22 LR 40gr', '.22 Long Rifle', 40.0, 1470.0, 0.130, 'Rimfire Powder', 2.1, 'Rimfire'),
    ((SELECT id FROM manufacturers WHERE name='Aguila'), 'Colibri 22 LR 20gr', '.22 Long Rifle', 20.0, 420.0, 0.095, 'Primer Only', 0.0, 'Rimfire'),
    ((SELECT id FROM manufacturers WHERE name='Aguila'), 'Super Colibri 22 LR 20gr', '.22 Long Rifle', 20.0, 500.0, 0.095, 'Minimal Powder', 0.2, 'Rimfire'),
    -- Aguila Pistol
    ((SELECT id FROM manufacturers WHERE name='Aguila'), '9mm 115gr FMJ', '9mm Luger', 115.0, 1150.0, 0.155, 'Universal', 5.0, 'Pistol'),
    ((SELECT id FROM manufacturers WHERE name='Aguila'), '9mm 124gr FMJ', '9mm Luger', 124.0, 1115.0, 0.165, 'Universal', 4.8, 'Pistol'),
    ((SELECT id FROM manufacturers WHERE name='Aguila'), '45 ACP 230gr FMJ', '.45 ACP', 230.0, 830.0, 0.195, 'Unique', 5.8, 'Pistol'),
    ((SELECT id FROM manufacturers WHERE name='Aguila'), '380 ACP 95gr FMJ', '.380 ACP', 95.0, 955.0, 0.130, 'HP-38', 3.5, 'Pistol'),
    ((SELECT id FROM manufacturers WHERE name='Aguila'), '38 Special 158gr SJSP', '.38 Special', 158.0, 755.0, 0.158, 'Unique', 4.5, 'Pistol'),
    -- Aguila Rifle
    ((SELECT id FROM manufacturers WHERE name='Aguila'), '5.56x45mm 62gr FMJ', '5.56x45mm NATO', 62.0, 3050.0, 0.307, 'TAC', 25.0, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Aguila'), '308 Win 150gr FMJBT', '.308 Winchester', 150.0, 2750.0, 0.435, 'Varget', 45.0, 'Rifle');

-- Insert Remington ammunition data
INSERT OR IGNORE INTO load_data (manufacturer_id, name, caliber, bullet_weight, velocity, bc, powder_type, powder_charge, category) VALUES
    -- Remington Rifle
    ((SELECT id FROM manufacturers WHERE name='Remington'), 'Core-Lokt 30-06 150gr', '.30-06 Springfield', 150.0, 2910.0, 0.314, 'IMR 4350', 58.0, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Remington'), 'Core-Lokt 308 Win 180gr', '.308 Winchester', 180.0, 2620.0, 0.383, 'IMR 4064', 41.5, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Remington'), 'Premier Match 223 Rem 77gr', '.223 Remington', 77.0, 2750.0, 0.372, 'Varget', 23.5, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Remington'), 'Core-Lokt 270 Win 130gr', '.270 Winchester', 130.0, 3060.0, 0.336, 'IMR 4831', 57.0, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Remington'), 'Core-Lokt 243 Win 100gr', '.243 Winchester', 100.0, 2960.0, 0.356, 'IMR 4350', 42.0, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Remington'), 'Core-Lokt 7mm Rem Mag 175gr', '7mm Remington Magnum', 175.0, 2860.0, 0.462, 'RL22', 66.0, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Remington'), 'Premier Match 6.5 Creedmoor 140gr', '6.5 Creedmoor', 140.0, 2700.0, 0.585, 'H4350', 41.0, 'Rifle'),
    ((SELECT id FROM manufacturers WHERE name='Remington'), 'Core-Lokt 300 Win Mag 180gr', '.300 Winchester Magnum', 180.0, 2960.0, 0.383, 'H1000', 73.0, 'Rifle'),
    -- Remington Pistol
    ((SELECT id FROM manufacturers WHERE name='Remington'), 'Golden Saber 9mm 124gr +P', '9mm Luger', 124.0, 1180.0, 0.165, 'Power Pistol', 6.3, 'Pistol'),
    ((SELECT id FROM manufacturers WHERE name='Remington'), 'Golden Saber 45 ACP 230gr', '.45 ACP', 230.0, 875.0, 0.195, 'Unique', 6.0, 'Pistol'),
    ((SELECT id FROM manufacturers WHERE name='Remington'), 'UMC 40 S&W 180gr FMJ', '.40 S&W', 180.0, 990.0, 0.164, 'Universal', 6.8, 'Pistol'),
    ((SELECT id FROM manufacturers WHERE name='Remington'), 'UMC 380 ACP 95gr FMJ', '.380 ACP', 95.0, 955.0, 0.130, 'HP-38', 3.6, 'Pistol'),
    ((SELECT id FROM manufacturers WHERE name='Remington'), 'HTP 38 Special +P 110gr', '.38 Special', 110.0, 995.0, 0.140, 'Unique', 5.5, 'Pistol'),
    -- Remington Rimfire
    ((SELECT id FROM manufacturers WHERE name='Remington'), 'Thunderbolt 22 LR 40gr', '.22 Long Rifle', 40.0, 1255.0, 0.138, 'Rimfire Powder', 1.7, 'Rimfire'),
    ((SELECT id FROM manufacturers WHERE name='Remington'), 'Golden Bullet 22 LR 36gr HP', '.22 Long Rifle', 36.0, 1280.0, 0.125, 'Rimfire Powder', 1.8, 'Rimfire');