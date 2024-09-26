use crate::app_data::AppData;
use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct IdBelanjaan {
    pub id: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Belanjaan {
    pub name: String,
    pub qty: i32,
    pub price: i64,
}

#[derive(sqlx::FromRow, Serialize, Debug, Clone)]
pub struct DaftarBelanjaan {
    pub id: i32,
    pub name: String,
    pub qty: i32,
    pub price: i64,
}

#[post("/daftar_belanjaan/delete")]
pub async fn delete_daftar_belanjaan(form: web::Form<IdBelanjaan>) -> impl Responder {
    let id = form.id.clone();
    println!("DELETING ID {}", id);

    // bikin connection pool
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .idle_timeout(std::time::Duration::new(15, 0))
        .connect("postgres://postgres:example@localhost:5432/b2rust")
        .await;

    // melakukan query
    let _ = sqlx::query_as::<_, ()>("DELETE FROM daftar_belanjaan WHERE id = $1")
        .bind(form.id.clone())
        .fetch_all(&pool.unwrap())
        .await;

    HttpResponse::Ok()
        .append_header(("HX-Redirect", "/daftar_belanjaan"))
        .body("")
}

#[post("/daftar_belanjaan")]
pub async fn add_daftar_belanjaan(
    form: web::Form<Belanjaan>,
    data: actix_web::web::Data<AppData>,
) -> impl Responder {
    // bikin connection pool
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .idle_timeout(std::time::Duration::new(15, 0))
        .connect("postgres://postgres:example@localhost:5432/b2rust")
        .await;

    // melakukan query
    let row = sqlx::query_as::<_, DaftarBelanjaan>(
        "INSERT INTO daftar_belanjaan (name, qty, price) VALUES ($1, $2, $3)",
    )
    .bind(form.name.clone())
    .bind(form.qty.clone())
    .bind(form.price.clone())
    .fetch_all(&pool.unwrap())
    .await;

    HttpResponse::Ok()
        .append_header(("HX-Redirect", "/daftar_belanjaan"))
        .body("")
}

#[get("/daftar_belanjaan")]
pub async fn list_daftar_belanjaan(
    data: actix_web::web::Data<AppData>,
    session: Session,
) -> impl Responder {
    let mut context = tera::Context::new();

    // bikin connection pool
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .idle_timeout(std::time::Duration::new(15, 0))
        .connect("postgres://postgres:example@localhost:5432/b2rust")
        .await;

    // melakukan query
    let row =
        sqlx::query_as::<_, DaftarBelanjaan>("SELECT id, name, qty, price from daftar_belanjaan")
            .fetch_all(&pool.unwrap())
            .await;

    if row.is_ok() {
        let res = row.unwrap();
        println!("{:?}", res);
        // passing daftar belanjaan ke html
        context.insert("daftar_belanjaan", &res);
    } else {
        let err = row.err().unwrap();
        println!("QUERY ERROR GAN!!! {}", err);
    }

    let rendered_page = data
        .template
        .render("daftar_belanjaan.html", &context)
        .unwrap();
    HttpResponse::Ok().body(rendered_page)
}
