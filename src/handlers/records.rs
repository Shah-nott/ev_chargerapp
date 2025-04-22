// src/handlers/records.rs

use actix_files::NamedFile;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize, Deserialize)]
pub struct ChargingRecord {
    record_id: String,
    charger_number: String,
    duration: String,
    date: String,
    energy_consumed: f32,
    cost: Option<f32>,
}

// Serve the records HTML page
pub async fn records_page() -> impl Responder {
    NamedFile::open("./static/records.html")
}

// Fetch charging records
pub async fn get_charging_records() -> impl Responder {
    let api_url = "http://127.0.0.1:8080/i/auth/pub/v1/chargers/records/getList";
    let client = Client::new();

    match client.post(api_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let records: Vec<ChargingRecord> = response.json().await.unwrap();
                HttpResponse::Ok().json(records)
            } else {
                HttpResponse::InternalServerError().body("Failed to fetch records")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to contact API"),
    }
}

// Add cost to the charging data
pub async fn add_cost(record: web::Json<ChargingRecord>) -> impl Responder {
    let api_url = "http://127.0.0.1:8080/i/auth/pub/v1/chargers/chargingDataPush";
    let client = Client::new();

    match client.post(api_url).json(&record.0).send().await {
        Ok(response) if response.status().is_success() => HttpResponse::Ok().body("Cost added successfully"),
        _ => HttpResponse::InternalServerError().body("Failed to add cost"),
    }
}

// Export the records to PDF (this is a placeholder; generating PDF is not covered here)
pub async fn export_to_pdf() -> impl Responder {
    HttpResponse::Ok().body("PDF export feature is coming soon!")
}
