use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use actix_files::NamedFile;
use actix_session::Session;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct RegisterForm {
    username: String,
    password: String,
    email:    Option<String>,
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn login_form() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./static/login.html")?)
}

pub async fn register_form() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./static/register.html")?)
}

pub async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Found().append_header(("Location", "/login")).finish()
}

// ---------- Register User ----------
pub async fn register_user(
    form:    web::Form<RegisterForm>,
    db_pool: web::Data<PgPool>
) -> impl Responder {
    
    let salt = SaltString::generate(&mut OsRng);
    let hash = match Argon2::default()
        .hash_password(form.password.as_bytes(), &salt)
    {
        Ok(h) => h.to_string(),
       Err(_) => return HttpResponse::InternalServerError().body("Hash error"),
    };

    let res = sqlx::query!(
        "INSERT INTO users (username,password_hash,email) VALUES ($1,$2,$3)",
        form.username, hash, form.email
    )
    .execute(db_pool.get_ref()).await;

    match res {
        Ok(_)  => HttpResponse::Found().append_header(("Location", "/login")).finish(),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Registration failed")
        }
    }
}

// ---------- Login User ----------
pub async fn login_user(
    form: web::Form<LoginForm>,
    db_pool: web::Data<PgPool>,
    session: Session,

) -> impl Responder {
    let rec  = sqlx::query!(
        "SELECT * FROM users WHERE username = $1",
        form.username
    )
    .fetch_optional(db_pool.get_ref())
    .await;

    match rec {
     Ok(Some(u)) if Argon2::default()
            .verify_password(form.password.as_bytes(), &PasswordHash::new(&u.password_hash).unwrap())
            .is_ok() =>
        {
            session.insert("username", &u.username).unwrap();
            HttpResponse::Found().append_header(("Location", "/home")).finish()
        }
        Ok(Some(_))  => HttpResponse::Unauthorized().body("Invalid credentials"),
        Ok(None)     => HttpResponse::NotFound().body("User not found"),
        Err(_)       => HttpResponse::InternalServerError().body("DB error"),
    }
}

// ---------- Configure Routes ----------
pub fn configure(cfg: &mut web::ServiceConfig) {
    // ── Login ─────────────────────────────────────────────
    cfg.service(
      web::resource("/login")
        .route(web::get().to(login_form))
        .route(web::post().to(login_user))
    );

    // ── Register ──────────────────────────────────────────
    cfg.service(
      web::resource("/register")
        .route(web::get().to(register_form))
        .route(web::post().to(register_user))
    );

    // ── Logout ────────────────────────────────────────────
    cfg.service(
      web::resource("/logout")
        .route(web::get().to(logout))
    );
}
