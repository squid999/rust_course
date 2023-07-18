#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate serde_json;
extern crate r2d2_diesel;

mod db;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web::JsonConfig, App, HttpServer};

    let conn_pool = db::establish_connection();

    HttpServer::new(move || {
        App::new()
            .data(conn_pool.clone())
            .data(JsonConfig::default().limit(4096))
            .configure(services::student::init_routes)
            .configure(services::stu_class::init_routes)
            .configure(services::class::init_routes)
    })
    .bind("0.0.0.0:8111")?
    .run()
    .await
}
