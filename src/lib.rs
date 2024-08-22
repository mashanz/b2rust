pub mod api;
pub mod pages;

use actix_web::{dev::Server, HttpServer, App, web};

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .service(pages::home::hello)
            .service(pages::echo::echo)
            .route("/hey", web::get().to(pages::hey::manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run();
    Ok(server)
}