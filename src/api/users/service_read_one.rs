use actix_web::{HttpResponse, Responder};

pub async fn read_one() -> impl Responder {
    HttpResponse::Ok().body("READ ONE")
}
