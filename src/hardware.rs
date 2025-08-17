use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Only use serialport on desktop with hardware feature
#[cfg(all(not(target_arch = "wasm32"), feature = "hardware"))]
use serialport::{SerialPort, SerialPortType};

#[derive(Clone, Serialize, Deserialize)]
pub struct RangefinderData {
    pub distance: f64,  // yards
    pub angle: f64,     // degrees
    pub timestamp: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WeatherData {
    pub temperature: f64,
    pub pressure: f64,
    pub humidity: f64,
    pub wind_speed: f64,
    pub wind_angle: f64,
    pub timestamp: String,
}

pub struct HardwareManager {
    rangefinder_data: Arc<Mutex<Option<RangefinderData>>>,
    weather_data: Arc<Mutex<Option<WeatherData>>>,
    rangefinder_connected: Arc<Mutex<bool>>,
    weather_connected: Arc<Mutex<bool>>,
    pub auto_connect: bool,
    pub auto_apply: bool,
}

impl Default for HardwareManager {
    fn default() -> Self {
        Self {
            rangefinder_data: Arc::new(Mutex::new(None)),
            weather_data: Arc::new(Mutex::new(None)),
            rangefinder_connected: Arc::new(Mutex::new(false)),
            weather_connected: Arc::new(Mutex::new(false)),
            auto_connect: false,
            auto_apply: false,
        }
    }
}

impl HardwareManager {
    pub fn connect_rangefinder(&mut self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Simulate a connection (replace with real BLE code if needed)
            *self.rangefinder_connected.lock().unwrap() = true;

            let data = self.rangefinder_data.clone();
            thread::spawn(move || loop {
                let reading = RangefinderData {
                    distance: 150.0 + rand::random::<f64>() * 50.0,
                    angle: -5.0 + rand::random::<f64>() * 10.0,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };
                *data.lock().unwrap() = Some(reading);
                thread::sleep(Duration::from_secs(1));
            });

            true
        }
        
        #[cfg(target_arch = "wasm32")]
        {
            // For WASM, just simulate without threads
            *self.rangefinder_connected.lock().unwrap() = true;
            let reading = RangefinderData {
                distance: 175.0,
                angle: 0.0,
                timestamp: chrono::Utc::now().to_rfc3339(),
            };
            *self.rangefinder_data.lock().unwrap() = Some(reading);
            true
        }
    }

    pub fn connect_weather_meter(&mut self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            *self.weather_connected.lock().unwrap() = true;

            let data = self.weather_data.clone();
            thread::spawn(move || loop {
                let reading = WeatherData {
                    temperature: 70.0 + rand::random::<f64>() * 20.0,
                    pressure: 29.5 + rand::random::<f64>() * 0.5,
                    humidity: 40.0 + rand::random::<f64>() * 30.0,
                    wind_speed: rand::random::<f64>() * 15.0,
                    wind_angle: rand::random::<f64>() * 360.0,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };
                *data.lock().unwrap() = Some(reading);
                thread::sleep(Duration::from_secs(2));
            });

            true
        }
        
        #[cfg(target_arch = "wasm32")]
        {
            *self.weather_connected.lock().unwrap() = true;
            let reading = WeatherData {
                temperature: 75.0,
                pressure: 29.92,
                humidity: 50.0,
                wind_speed: 5.0,
                wind_angle: 180.0,
                timestamp: chrono::Utc::now().to_rfc3339(),
            };
            *self.weather_data.lock().unwrap() = Some(reading);
            true
        }
    }

    pub fn rangefinder_connected(&self) -> bool {
        *self.rangefinder_connected.lock().unwrap()
    }

    pub fn weather_meter_connected(&self) -> bool {
        *self.weather_connected.lock().unwrap()
    }

    pub fn get_rangefinder_data(&self) -> Option<RangefinderData> {
        self.rangefinder_data.lock().unwrap().clone()
    }

    pub fn get_weather_data(&self) -> Option<WeatherData> {
        self.weather_data.lock().unwrap().clone()
    }

    pub fn refresh_all(&mut self) {
        if !self.rangefinder_connected() {
            let _ = self.connect_rangefinder();
        }
        if !self.weather_meter_connected() {
            let _ = self.connect_weather_meter();
        }
    }
}