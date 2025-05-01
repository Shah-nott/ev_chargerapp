// src/routes/mod.rs

pub mod auth;

use actix_web::web;

// Auth handlers
use crate::routes::auth::{login_form, register_form, login_user, register_user, logout};

// Page‐level handlers
use crate::handlers::login::login_page;
use crate::handlers::home::{home_page, home_data, get_charger_status, start_stop_charging};
use crate::handlers::add_device::{add_device_page, scan_devices, connect_device, get_status as get_device_status};
use crate::handlers::schedule::{schedule_page, get_schedule_list, add_schedule, add_recurring_schedule};
use crate::handlers::records::{records_page, get_charging_records, add_cost, export_to_pdf};
use crate::handlers::settings::{
    settings_page, update_nickname, set_network, set_max_current,
    set_charge_mode, set_led_brightness, set_locking_mode, add_family_sharing,
};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    auth::configure(cfg);


    // ── Static pages ───────────────────────────────────────────────────────────────
    cfg.route("/",      web::get().to(login_page))
       .route("/home",  web::get().to(home_page))
       // AJAX for user data on home
       .route("/api/home", web::get().to(home_data));

    // ── Device / Charger ──────────────────────────────────────────────────────────
    cfg.route("/add_device",     web::get().to(add_device_page))
       .route("/scan",           web::get().to(scan_devices))
       .route("/connect",        web::post().to(connect_device))
       .route("/status",         web::get().to(get_device_status))
       .route("/home/status",    web::get().to(get_charger_status))
       .route("/home/start_stop",web::post().to(start_stop_charging));

    // ── Schedule ─────────────────────────────────────────────────────────────────
    cfg.route("/schedule",               web::get().to(schedule_page))
       .route("/schedule/list",          web::get().to(get_schedule_list))
       .route("/schedule/add",           web::post().to(add_schedule))
       .route("/schedule/add_recurring", web::post().to(add_recurring_schedule));

    // ── Records ─────────────────────────────────────────────────────────────────
    cfg.route("/records",          web::get().to(records_page))
       .route("/records/list",     web::get().to(get_charging_records))
       .route("/records/add_cost", web::post().to(add_cost))
       .route("/records/export",   web::get().to(export_to_pdf));

    // ── Settings ────────────────────────────────────────────────────────────────
    cfg.route("/settings",                  web::get().to(settings_page))
       .route("/settings/updateNickname",   web::post().to(update_nickname))
       .route("/settings/setNetwork",       web::post().to(set_network))
       .route("/settings/setMaxCurrent",    web::post().to(set_max_current))
       .route("/settings/setChargeMode",    web::post().to(set_charge_mode))
       .route("/settings/setLedBrightness", web::post().to(set_led_brightness))
       .route("/settings/setLockingMode",   web::post().to(set_locking_mode))
       .route("/settings/addFamilySharing", web::post().to(add_family_sharing));
}
