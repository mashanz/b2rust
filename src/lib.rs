pub mod app_data;
pub mod api;
pub mod pages;

use actix_web::{dev::Server, HttpServer, HttpRequest, App, web, get, Error};
use crate::app_data::AppData;

#[get("/assets/{filename:.*}")]
async fn cdn_alaala(req: HttpRequest) -> Result<actix_files::NamedFile, Error> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    let file = actix_files::NamedFile::open(path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(actix_web::http::header::ContentDisposition {
            disposition: actix_web::http::header::DispositionType::Attachment,
            parameters: vec![],
        })
    )
}

pub fn run() -> Result<Server, std::io::Error> {
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
