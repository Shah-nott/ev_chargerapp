use btleplug::api::{Central as _, Manager as _, Peripheral as _, ScanFilter, CharPropFlags};
use btleplug::platform::Manager;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio;
use futures;

#[derive(Serialize, Deserialize, Clone)] // Add Deserialize
pub struct DeviceStatus {
    pub connected: bool,
    pub device_name: Option<String>,
    pub last_connected_device: Option<String>,
    pub data_received: Option<String>, // Add a field to track received data
    pub charging: bool,
    pub charge_mode: Option<String>,  // Fast, Eco, Green
    pub charge_speed: Option<String>, // E.g. Regular, Fast, Slow
    pub amount_spent: Option<f32>,    // Amount spent on last charge
}

type SharedState = Arc<Mutex<DeviceStatus>>;

pub async fn scan_devices() -> Vec<String> {
    let manager = Manager::new().await;
    let mut scanned_devices = vec![];
    if let Ok(manager) = manager {
        if let Ok(adapters) = manager.adapters().await {
            if let Some(central) = adapters.into_iter().nth(0) {
                if central.start_scan(ScanFilter::default()).await.is_ok() {
                    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                    if let Ok(peripherals) = central.peripherals().await {
                        for peripheral in peripherals {
                            if let Some(properties) = peripheral.properties().await.unwrap() {
                                if let Some(name) = &properties.local_name {
                                    scanned_devices.push(name.clone());
                                }
                            }
                        }
                    } else {
                        println!("Failed to get peripherals");
                    }
                } else {
                    println!("Failed to start Bluetooth scan");
                }
            } else {
                println!("No Bluetooth adapters found");
            }
        } else {
            println!("Failed to get Bluetooth adapters");
        }
    } else {
        println!("Failed to initialize Bluetooth manager");
    }
    scanned_devices
}


pub async fn read_data_from_device(device_name: &str, shared_state: SharedState) {
    if let Ok(manager) = Manager::new().await {
        if let Ok(adapters) = manager.adapters().await {
            if let Some(central) = adapters.into_iter().nth(0) {
                if central.start_scan(ScanFilter::default()).await.is_ok() {
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    if let Ok(peripherals) = central.peripherals().await {
                        for peripheral in peripherals {
                            if let Some(properties) = peripheral.properties().await.unwrap() {
                                if let Some(name) = &properties.local_name {
                                    if name == device_name {
                                        if peripheral.connect().await.is_ok() {
                                            let characteristics = peripheral.characteristics();
                                            for characteristic in characteristics {
                                                if characteristic.properties.contains(CharPropFlags::READ) {
                                                    if let Ok(value) = peripheral.read(&characteristic).await {
                                                        let data_str = format!("{:?}", value);
                                                        let mut state = shared_state.lock().unwrap();
                                                        state.data_received = Some(data_str);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
