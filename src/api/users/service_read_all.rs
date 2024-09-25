use actix_web::{HttpResponse, Responder};

pub async fn read_all() -> impl Responder {
    HttpResponse::Ok().body("READ ALL")
}
