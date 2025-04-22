use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use actix_files::NamedFile;

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub nickname: Option<String>,
    pub line_type: Option<String>,
    pub max_current: Option<u32>,
    pub charge_mode: Option<String>,
    pub led_brightness: Option<u8>,
    pub locking_mode: Option<String>,
    pub family_member_id: Option<String>,
}

// Render the settings page
pub async fn settings_page() -> impl Responder {
    NamedFile::open("./static/settings.html")
}

// Update EV Charger Nickname
pub async fn update_nickname(info: web::Json<Settings>) -> impl Responder {
    let api_url = "http://127.0.0.1:8080/i/auth/pub/v1/chargers/settings";
    let client = Client::new();

    match client.post(api_url)
        .json(&*info)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            HttpResponse::Ok().json(format!("Nickname updated to {}", info.nickname.clone().unwrap_or_default()))
        }
        _ => HttpResponse::InternalServerError().body("Failed to update nickname"),
    }
}

// Set EV Charger Network (Bluetooth/Wi-Fi)
pub async fn set_network(info: web::Json<Settings>) -> impl Responder {
    let api_url = "http://127.0.0.1:8080/i/auth/pub/v1/chargers/settings/setNetwork";
    let client = Client::new();

    match client.post(api_url)
        .json(&*info)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            HttpResponse::Ok().json(format!("Network set to {}", info.line_type.clone().unwrap_or_default()))
        }
        _ => HttpResponse::InternalServerError().body("Failed to set network type"),
    }
}

// Set Maximum Current for the Charger
pub async fn set_max_current(info: web::Json<Settings>) -> impl Responder {
    let api_url = "http://127.0.0.1:8080/i/auth/pub/v1/chargers/settings/setMaxCurrent";
    let client = Client::new();

    match client.post(api_url)
        .json(&*info)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            HttpResponse::Ok().json(format!("Max current set to {} A", info.max_current.unwrap_or_default()))
        }
        _ => HttpResponse::InternalServerError().body("Failed to set max current"),
    }
}

// Set EV Charger Charge Mode
pub async fn set_charge_mode(info: web::Json<Settings>) -> impl Responder {
    let api_url = "http://127.0.0.1:8080/i/auth/pub/v1/chargers/settings/setLoadBalance";
    let client = Client::new();

    match client.post(api_url)
        .json(&*info)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            HttpResponse::Ok().json(format!("Charge mode set to {}", info.charge_mode.clone().unwrap_or_default()))
        }
        _ => HttpResponse::InternalServerError().body("Failed to set charge mode"),
    }
}

// Set LED Brightness for the Charger
pub async fn set_led_brightness(info: web::Json<Settings>) -> impl Responder {
    let api_url = "http://127.0.0.1:8080/i/auth/pub/v1/chargers/settings/setLed";
    let client = Client::new();

    match client.post(api_url)
        .json(&*info)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            HttpResponse::Ok().json(format!("LED brightness set to {}", info.led_brightness.unwrap_or_default()))
        }
        _ => HttpResponse::InternalServerError().body("Failed to set LED brightness"),
    }
}

// Set Locking Mode for the Charger
pub async fn set_locking_mode(info: web::Json<Settings>) -> impl Responder {
    let api_url = "http://127.0.0.1:8080/i/auth/pub/v1/chargers/settings/setWorkMode";
    let client = Client::new();

    match client.post(api_url)
        .json(&*info)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            HttpResponse::Ok().json(format!("Locking mode set to {}", info.locking_mode.clone().unwrap_or_default()))
        }
        _ => HttpResponse::InternalServerError().body("Failed to set locking mode"),
    }
}

// Add Family Sharing for the Charger
pub async fn add_family_sharing(info: web::Json<Settings>) -> impl Responder {
    let api_url = "http://127.0.0.1:8080/i/auth/pub/v1/users/bindChargers";
    let client = Client::new();

    match client.post(api_url)
        .json(&*info)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            HttpResponse::Ok().json(format!("Family member with ID {} added successfully", info.family_member_id.clone().unwrap_or_default()))
        }
        _ => HttpResponse::InternalServerError().body("Failed to add family sharing"),
    }
}
