use actix_files::NamedFile;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use crate::bluetooth::{scan_devices as scan_bt_devices, read_data_from_device, DeviceStatus};
use tokio;

type SharedState = Arc<Mutex<DeviceStatus>>;

pub async fn add_device_page() -> impl Responder {
    NamedFile::open("./static/add_device.html")
}

#[derive(Deserialize)]
pub struct ConnectInfo {
    sn_number: String,
}

pub async fn scan_devices(_data: web::Data<SharedState>) -> impl Responder {
    let devices = scan_bt_devices().await;
    HttpResponse::Ok().json(devices)
}

pub async fn connect_device(data: web::Data<SharedState>, connect_info: web::Form<ConnectInfo>) -> impl Responder {
    let sn_number = &connect_info.sn_number;

    // Convert `web::Data<Arc<Mutex<DeviceStatus>>>` to `Arc<Mutex<DeviceStatus>>` by dereferencing
    let device_status = Arc::clone(&data);

    // Scan and connect asynchronously
    let sn = sn_number.clone().to_string();
    tokio::spawn(async move {
        let devices = scan_bt_devices().await;
        if devices.contains(&sn) {
            {
                let mut status = device_status.lock().unwrap();
                status.connected = true;
                status.device_name = Some(sn.clone());
                status.last_connected_device = Some(sn.clone());
            }
            read_data_from_device(&sn, Arc::clone(&device_status)).await;
        } else {
            let mut status = device_status.lock().unwrap();
            status.connected = false;
            status.device_name = None;
        }
    });

    HttpResponse::Ok().json("Connecting to device...")
}

pub async fn get_status(data: web::Data<SharedState>) -> impl Responder {
    let device_status = data.lock().unwrap().clone();
    HttpResponse::Ok().json(device_status)
}
