use actix_web::{Responder, HttpResponse};

pub async fn read_one() -> impl Responder {
    HttpResponse::Ok().body("READ ONE")
}