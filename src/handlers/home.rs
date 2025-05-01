use crate::bluetooth::DeviceStatus; // Import DeviceStatus from the bluetooth module
use actix_files::NamedFile;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use reqwest::Client;
use actix_session::Session;
use sqlx::PgPool;


type SharedState = Arc<Mutex<DeviceStatus>>;

#[derive(Serialize, Deserialize, Clone)] // Add Deserialize trait here
pub struct StartStopChargingRequest {
    pub start: bool,
}
#[derive(serde::Serialize)]
pub struct UserResponse {
    pub username: String,
    pub email: Option<String>,
}


pub async fn home_page(session: Session, req: HttpRequest)
    -> actix_web::Result<HttpResponse>
{
    match session.get::<String>("username") {
        Ok(Some(_username)) => {
            // logged in: serve home.html
            let file = NamedFile::open("./static/home.html")?;
            Ok(file.into_response(&req))
        }
        _ => {
            // no session or error: bounce to /login
            Ok(HttpResponse::Found()
                .append_header(("Location", "/login"))
                .finish())
        }
    }
}




pub async fn home_data(session: Session, db: web::Data<PgPool>) -> impl Responder {
    match session.get::<String>("username") {
        Ok(Some(username)) => {
            let row = sqlx::query!("SELECT username, email FROM users WHERE username = $1", username)
                .fetch_one(db.get_ref())
                .await;
            match row {
                Ok(u) => HttpResponse::Ok().json(UserResponse { username: u.username, email: u.email }),
                Err(_) => HttpResponse::InternalServerError().body("DB error"),
            }
        }
        _ => HttpResponse::Unauthorized().body("Not logged in"),
    }
}


// Query current charger status from the backend API
pub async fn get_charger_status(data: web::Data<SharedState>) -> impl Responder {
    let _device_status = data.lock().unwrap().clone();

    // Here we'll use the API to query the latest charging data
    let api_url = "http://127.0.0.1:8080/i/auth/pub/v1/chargers/queryChargingData"; // Replace with the actual API endpoint
    let client = Client::new();

    match client.post(api_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let updated_status = response.json::<DeviceStatus>().await.unwrap();
                
                // Update the shared state with the API response using cloned value
                let mut device_status = data.lock().unwrap();
                *device_status = updated_status.clone();  // Use clone here to avoid ownership issues

                HttpResponse::Ok().json(updated_status) // Return cloned value
            } else {
                HttpResponse::InternalServerError().body("Failed to get charger status")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to contact API"),
    }
}

// Start or Stop charging
pub async fn start_stop_charging(data: web::Data<SharedState>, req: web::Json<StartStopChargingRequest>) -> impl Responder {
    let api_url = if req.start {
        "http://127.0.0.1:8080/i/auth/pub/v1/chargers/start"
    } else {
        "http://127.0.0.1:8080/i/auth/pub/v1/chargers/stop"
    };

    let client = Client::new();
    match client.post(api_url).send().await {
        Ok(response) if response.status().is_success() => {
            let mut device_status = data.lock().unwrap();
            device_status.charging = req.start;
            HttpResponse::Ok().json("Charging status updated successfully")
        }
        _ => HttpResponse::InternalServerError().body("Failed to update charging status"),
    }
}
