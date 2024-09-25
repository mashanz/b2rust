use actix_web::{HttpResponse, Responder};

pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("UPDATE")
}
