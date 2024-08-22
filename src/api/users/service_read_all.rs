use actix_web::{Responder, HttpResponse};

pub async fn read_all() -> impl Responder {
    HttpResponse::Ok().body("READ ALL")
}