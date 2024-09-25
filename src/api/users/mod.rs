use actix_web::{web, web::delete, web::get, web::patch, web::post, web::resource, Scope};
mod service_create;
mod service_delete;
mod service_read_all;
mod service_read_one;
mod service_update;

pub fn service() -> Scope {
    web::scope("/api/users")
        .service(resource("/").route(get().to(service_read_all::read_all))) // READ all users
        .service(resource("/id/{user_id}").route(get().to(service_read_one::read_one))) // READ single users
        .service(resource("/create").route(post().to(service_create::create))) // CREATE users
        .service(resource("/id/{user_id}/update").route(patch().to(service_update::update))) // UPDATE users
        .service(resource("/id/{user_id}/delete").route(delete().to(service_delete::delete)))
    // DELETE users
}
