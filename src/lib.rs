pub mod app_data;
pub mod api;
pub mod pages;

use actix_web::{dev::Server, HttpServer, HttpRequest, App, web, get, Error, cookie::Key};
use crate::app_data::AppData;
use actix_session::{SessionMiddleware, storage::RedisActorSessionStore};
use actix_web::middleware::Logger;
use env_logger::Env;

#[get("/assets/{filename:.*}")]
async fn cdn_alaala(req: HttpRequest) -> Result<actix_files::NamedFile, Error> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    let file = actix_files::NamedFile::open(path)?;
    Ok(file)
}

pub fn run() -> Result<Server, std::io::Error> {
    // Logging
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // auto generate identifier untuk redis session state
    let secret_key = Key::generate();

    // create server
    let server = HttpServer::new(move || {

        // Load Templates
        let tera_data = match tera::Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Tidak bisa parsing template karena: {}", e);
                ::std::process::exit(1);
            }
        };

        // Running Apps
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(SessionMiddleware::new(
                RedisActorSessionStore::new("127.0.0.1:6379"),
                secret_key.clone()))
            .app_data(actix_web::web::Data::new(AppData {
                template: tera_data.clone(),
            }))
            .service(pages::home::hello)
            .service(pages::echo::echo)
            .service(api::users::service())
            .service(cdn_alaala)
            .route("/hey", web::get().to(pages::hey::manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run();
    Ok(server)
}
