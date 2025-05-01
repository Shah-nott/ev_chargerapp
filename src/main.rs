mod bluetooth;
mod handlers;
mod routes;

use crate::routes::init_routes;
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use crate::bluetooth::DeviceStatus;



type SharedState = Arc<Mutex<DeviceStatus>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load env vars from .env

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let secret_key = Key::generate();
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("❌ Failed to connect to the database");

    let device_status = Arc::new(Mutex::new(DeviceStatus {
        connected: false,
        device_name: None,
        last_connected_device: None,
        charging: false,
        charge_mode: None,
        charge_speed: None,
        amount_spent: None,
        data_received: None, // Added the missing field
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(device_status.clone()))
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    secret_key.clone(),
                )
                .cookie_name("ev-session".into())
                .cookie_secure(true)
                .build()
            )
            .configure(init_routes)
           // serve CSS
           .service(
               fs::Files::new("/static/css", "./static/css")
                   .show_files_listing()
                   .use_last_modified(true)
           )
           // serve JS
           .service(
               fs::Files::new("/static/js", "./static/js")
                   .show_files_listing()
                   .use_last_modified(true)
           )
    })                          // ← closes the closure you passed to HttpServer::new
    .bind("0.0.0.0:8080")?
    .run()
    .await
}