use actix_web::{get, HttpResponse, Responder};
use crate::app_data::AppData;
use actix_session::Session;

#[get("/")]
pub async fn hello(data: actix_web::web::Data<AppData>, session: Session) -> impl Responder {
    // create new context
    let mut context = tera::Context::new();

    if let Some(count) = session.get::<i32>("konter").unwrap() {
        println!("SESI value: {}", count);
        let _ = session.insert("konter", count + 1);
        context.insert("val", &format!("{}", count + 1));
    } else {
        session.insert("konter", 1);
        session.insert("email", "email@mashanz.com");
        session.insert("name", "papanberjalan");
        context.insert("val", &format!("{}", 1));
    }
    // bikin rendereing template
    let rendered_page = data.template.render("index.html", &context).unwrap();

    // return rendered template
    HttpResponse::Ok().body(rendered_page)
}
