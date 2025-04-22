use actix_files::NamedFile;
use actix_web::{Responder};



pub async fn login_page() -> impl Responder {
    NamedFile::open("./static/login.html")
}
