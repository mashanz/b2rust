use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct UserRequestBody {
    pub name: String,
    pub email: String,
}

pub async fn create(json: web::Json<UserRequestBody>) -> impl Responder {
    HttpResponse::Ok().body(format!("CREATE: {}", json.name))
}
