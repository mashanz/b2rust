use crate::app_data::AppData;
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: String,
}

#[post("/")]
pub async fn login_handler(session: Session, form: web::Form<LoginForm>) -> impl Responder {
    // nampilin data hasil sent form data
    println!("email {}", form.email);
    println!("password {}", form.password);

    // bikin connection pool
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .idle_timeout(std::time::Duration::new(15, 0))
        .connect("postgres://postgres:example@localhost:5432/b2rust")
        .await;

    // melakukan query
    let row = sqlx::query_as::<_, User>(
        "SELECT id, email, name, role from users where email = $1 and password = $2",
    )
    .bind(form.email.clone())
    .bind(form.password.clone())
    .fetch_one(&pool.unwrap())
    .await;

    // kalo oke, simpan return di server session
    if row.is_ok() {
        let res = row.unwrap();
        session.insert("id", res.id.to_string());
        session.insert("email", res.email);
        session.insert("name", res.name);
        session.insert("role", res.role);
    } else {
        let err = row.err().unwrap();
        println!("ERROR GAN!!! {}", err);
    }

    // testing session
    if let Some(role) = session.get::<String>("role").unwrap() {
        // jika record ada
        println!("email {}", role);
        if role == "admin" {
            // admin
        } else {
            // bukan admin
        }
    } else {
        // jika record tidak ada
    }
    HttpResponse::Ok().body("")
}

#[get("/")]
pub async fn hello(data: actix_web::web::Data<AppData>, session: Session) -> impl Responder {
    // create new context
    let mut context = tera::Context::new();

    if let Some(count) = session.get::<i32>("konter").unwrap() {
        println!("SESI value: {}", count);
        let _ = session.insert("konter", count + 1);
        context.insert("val", &format!("{}", count + 1));
    } else {
        context.insert("val", &format!("{}", 1));
    }
    // bikin rendereing template
    let rendered_page = data.template.render("index.html", &context).unwrap();

    // return rendered template
    HttpResponse::Ok().body(rendered_page)
}
