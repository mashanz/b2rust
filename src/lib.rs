pub mod api;
pub mod pages;

use actix_web::{dev::Server, HttpServer, App, web};

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .service(pages::home::hello)
            .service(pages::echo::echo)
            .service(api::users::service())
            .route("/hey", web::get().to(pages::hey::manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run();
    Ok(server)
}