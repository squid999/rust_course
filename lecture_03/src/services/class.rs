use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::db::{
    models::Student,
    models::{Class, StuClass},
    DbPool,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassForm {
    class_id: String,
    name: Option<String>,
    stu_list: Option<Vec<Student>>,
}

pub fn create(cls_form: web::Json<ClassForm>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();
    println!("{:?}", cls_form);
    match Class::create(&cls_form.class_id, cls_form.name.as_deref(), &conn) {
        Some(cls) => HttpResponse::Ok().json(cls),
        _ => HttpResponse::InternalServerError().json("Could not create class"),
    }
    // HttpResponse::Ok().json("success")
}

pub fn index(pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();

    HttpResponse::Ok().json(Class::list(&conn))
    // HttpResponse::Ok().json("[]")
}

pub fn get(class_id: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();

    match Class::by_clsid(&class_id, &conn) {
        Some(cls) => {
            println!();
            HttpResponse::Ok().json(cls)
        }
        _ => HttpResponse::NotFound().json("Not Found"),
    }
}
pub fn del(class_id: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();

    match Class::remove(&class_id, &conn) {
        Ok(_) => match StuClass::remove_cls(&class_id, &conn) {
            Ok(_) => HttpResponse::Ok().json("success"),
            Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
        },
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    /*
     * index: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/classes
     * get: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/classes/<clsid>
     * delete: curl -i -X DELETE -H "Content-Type: application/json" http://localhost:5000/classes/<clsid>
     * post: curl -i -X POST -H "Content-Type: application/json" -d '{"email":"xxx", "phone": "yyy"}' http://localhost:5000/classes
     */

    cfg.service(
        web::resource("api/v1/classes")
            .route(web::post().to(create))
            .route(web::get().to(index)),
    )
    .service(
        web::resource("api/v1/classes/{class_id}")
            .route(web::get().to(get))
            .route(web::delete().to(del)),
    );
}
