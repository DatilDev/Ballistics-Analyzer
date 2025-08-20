use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[cfg(feature = "hardware")]
use serialport::{available_ports, SerialPort};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareDevice {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub connected: bool,
    pub battery_level: Option<f32>,
    pub last_reading: Option<DeviceReading>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    Rangefinder,
    WeatherMeter,
    Chronograph,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceReading {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: ReadingData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReadingData {
    Range {
        distance: f64,
        angle: Option<f64>,
        temperature: Option<f64>,
    },
    Weather {
        temperature: f64,
        pressure: f64,
        humidity: f64,
        wind_speed: Option<f64>,
        wind_direction: Option<f64>,
    },
    Velocity {
        fps: f64,
        shot_number: u32,
    },
}

pub struct HardwareManager {
    devices: Arc<Mutex<Vec<HardwareDevice>>>,
    #[cfg(feature = "hardware")]
    serial_ports: Arc<Mutex<Vec<Box<dyn SerialPort>>>>,
}

#[allow(dead_code)]
impl HardwareManager {
    pub fn new() -> Self {
        Self {
            devices: Arc::new(Mutex::new(Vec::new())),
            #[cfg(feature = "hardware")]
            serial_ports: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn scan_devices(&mut self) -> Result<Vec<HardwareDevice>> {
        let mut found_devices = Vec::new();

        #[cfg(feature = "hardware")]
        {
            // Scan serial ports
            if let Ok(ports) = available_ports() {
                for port in ports {
                    let device = HardwareDevice {
                        id: port.port_name.clone(),
                        name: self.identify_device(&port.port_name),
                        device_type: self.detect_device_type(&port.port_name),
                        connected: false,
                        battery_level: None,
                        last_reading: None,
                    };
                    found_devices.push(device);
                }
            }

            // Scan Bluetooth devices (platform specific)
            #[cfg(target_os = "windows")]
            self.scan_bluetooth_windows(&mut found_devices)?;
            
            #[cfg(target_os = "macos")]
            self.scan_bluetooth_macos(&mut found_devices)?;
            
            #[cfg(target_os = "linux")]
            self.scan_bluetooth_linux(&mut found_devices)?;
        }

        *self.devices.lock().unwrap() = found_devices.clone();
        Ok(found_devices)
    }

    pub fn connect_device(&mut self, device_id: &str) -> Result<()> {
        #[cfg(feature = "hardware")]
        {
            let mut devices = self.devices.lock().unwrap();
            if let Some(device) = devices.iter_mut().find(|d| d.id == device_id) {
                // Open serial connection
                match serialport::new(&device.id, 9600)
                    .timeout(Duration::from_millis(100))
                    .open()
                {
                    Ok(port) => {
                        self.serial_ports.lock().unwrap().push(port);
                        device.connected = true;
                        Ok(())
                    }
                    Err(e) => Err(anyhow::anyhow!("Failed to connect: {}", e)),
                }
            } else {
                Err(anyhow::anyhow!("Device not found"))
            }
        }
        
        #[cfg(not(feature = "hardware"))]
        {
            Err(anyhow::anyhow!("Hardware support not enabled"))
        }
    }

    pub fn disconnect_device(&mut self, device_id: &str) -> Result<()> {
        let mut devices = self.devices.lock().unwrap();
        if let Some(device) = devices.iter_mut().find(|d| d.id == device_id) {
            device.connected = false;
            // Port will be dropped automatically
            Ok(())
        } else {
            Err(anyhow::anyhow!("Device not found"))
        }
    }

    pub fn read_device(&self, device_id: &str) -> Result<Option<DeviceReading>> {
        let devices = self.devices.lock().unwrap();
        if let Some(device) = devices.iter().find(|d| d.id == device_id && d.connected) {
            // Simulate reading based on device type
            let reading = match device.device_type {
                DeviceType::Rangefinder => DeviceReading {
                    timestamp: chrono::Utc::now(),
                    data: ReadingData::Range {
                        distance: 543.2,
                        angle: Some(-5.3),
                        temperature: Some(72.0),
                    },
                },
                DeviceType::WeatherMeter => DeviceReading {
                    timestamp: chrono::Utc::now(),
                    data: ReadingData::Weather {
                        temperature: 68.5,
                        pressure: 29.92,
                        humidity: 45.0,
                        wind_speed: Some(8.5),
                        wind_direction: Some(270.0),
                    },
                },
                DeviceType::Chronograph => DeviceReading {
                    timestamp: chrono::Utc::now(),
                    data: ReadingData::Velocity {
                        fps: 2650.0,
                        shot_number: 1,
                    },
                },
                _ => return Ok(None),
            };
            Ok(Some(reading))
        } else {
            Ok(None)
        }
    }

    fn identify_device(&self, port_name: &str) -> String {
        // Parse device name from port
        if port_name.contains("KILO") {
            "Sig KILO Rangefinder".to_string()
        } else if port_name.contains("Kestrel") {
            "Kestrel Weather Meter".to_string()
        } else if port_name.contains("LabRadar") {
            "LabRadar Chronograph".to_string()
        } else {
            format!("Unknown Device ({})", port_name)
        }
    }

    fn detect_device_type(&self, port_name: &str) -> DeviceType {
        if port_name.contains("KILO") || port_name.contains("Leica") {
            DeviceType::Rangefinder
        } else if port_name.contains("Kestrel") || port_name.contains("Weather") {
            DeviceType::WeatherMeter
        } else if port_name.contains("LabRadar") || port_name.contains("Chrono") {
            DeviceType::Chronograph
        } else {
            DeviceType::Unknown
        }
    }

    #[cfg(all(feature = "hardware", target_os = "windows"))]
    fn scan_bluetooth_windows(&self, devices: &mut Vec<HardwareDevice>) -> Result<()> {
        // Windows Bluetooth implementation
        Ok(())
    }

    #[cfg(all(feature = "hardware", target_os = "macos"))]
    fn scan_bluetooth_macos(&self, devices: &mut Vec<HardwareDevice>) -> Result<()> {
        // macOS CoreBluetooth implementation
        Ok(())
    }

    #[cfg(all(feature = "hardware", target_os = "linux"))]
    fn scan_bluetooth_linux(&self, devices: &mut Vec<HardwareDevice>) -> Result<()> {
        // Linux BlueZ implementation
        Ok(())
    }

    pub fn get_connected_devices(&self) -> Vec<HardwareDevice> {
        self.devices
            .lock()
            .unwrap()
            .iter()
            .filter(|d| d.connected)
            .cloned()
            .collect()
    }
}

impl Default for HardwareManager {
    fn default() -> Self {
        Self::new()
    }
}