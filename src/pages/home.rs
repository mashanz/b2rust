use actix_web::{get, HttpResponse, Responder};
use crate::app_data::AppData;

#[get("/")]
pub async fn hello(data: actix_web::web::Data<AppData>) -> impl Responder {
    // create new context
    let mut context = tera::Context::new();


    // bikin rendereing template
    let rendered_page = data.template.render("index.html", &context).unwrap();
    
    // return rendered template
    HttpResponse::Ok().body(rendered_page)
}
