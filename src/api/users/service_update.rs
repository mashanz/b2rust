use actix_web::{Responder, HttpResponse};

pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("UPDATE")
}