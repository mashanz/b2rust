use actix_web::{Responder, HttpResponse};

pub async fn delete() -> impl Responder {
    HttpResponse::Ok().body("DELETE")
}