use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use actix_files::NamedFile;



#[derive(Serialize, Deserialize, Clone)]
pub struct Schedule {
    pub charger_id: String,
    pub start_time: String,  // e.g., "2024-11-14T08:00:00Z"
    pub end_time: String,    // e.g., "2024-11-14T10:00:00Z"
    pub recurring: Option<bool>,  // Optional field for recurring schedules
}

// Render the schedule page
pub async fn schedule_page() -> impl Responder {
    NamedFile::open("./static/schedule.html")
}
// Get the list of schedules
pub async fn get_schedule_list() -> impl Responder {
    let api_url = "http://127.0.0.1:8080/i/auth/pub/v1/chargers/times/getList";
    let client = Client::new();

    match client.post(api_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let schedule_list = response.json::<Vec<Schedule>>().await.unwrap();
                HttpResponse::Ok().json(schedule_list)
            } else {
                HttpResponse::InternalServerError().body("Failed to get schedule list")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to contact API"),
    }
}

// Add a new schedule
pub async fn add_schedule(schedule: web::Json<Schedule>) -> impl Responder {
    let api_url = "http://127.0.0.1:8080/i/auth/pub/v1/chargers/times/add";
    let client = Client::new();

    match client.post(api_url)
        .json(&*schedule)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            HttpResponse::Ok().json("Schedule added successfully")
        }
        _ => HttpResponse::InternalServerError().body("Failed to add schedule"),
    }
}

// Add a recurring schedule
pub async fn add_recurring_schedule(schedule: web::Json<Schedule>) -> impl Responder {
    let api_url = "http://127.0.0.1:8080/i/auth/pub/v1/chargers/times/add";
    let client = Client::new();

    match client.post(api_url)
        .json(&*schedule)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            HttpResponse::Ok().json("Recurring schedule added successfully")
        }
        _ => HttpResponse::InternalServerError().body("Failed to add recurring schedule"),
    }
}
