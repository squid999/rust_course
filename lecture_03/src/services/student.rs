use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::db::{models::Student, DbPool};

#[derive(Serialize, Deserialize)]
pub struct StudentForm {
    name: Option<String>,
}

pub fn create(stu_form: web::Json<StudentForm>, pool: web::Data<DbPool>) -> HttpResponse {
    // let conn = pool.get().unwrap();

    // match Student::create(stu_form.name.as_deref(), &conn) {
    //     Some(user) => HttpResponse::Ok().json(user),
    //     _ => HttpResponse::InternalServerError().json("Could not create user"),
    // }
    HttpResponse::Ok().json("success")
}

pub fn index(pool: web::Data<DbPool>) -> HttpResponse {
    // let conn = pool.get().unwrap();

    // HttpResponse::Ok().json(Student::list(&conn))
    HttpResponse::Ok().json("[]")
}

pub fn get(id: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    // let conn = pool.get().unwrap();

    // match Student::by_id(&id, &conn) {
    //     Some(stu) => HttpResponse::Ok().json(stu),
    //     _ => HttpResponse::NotFound().json("Not Found"),
    // }
    println!("{:?}", id);
    HttpResponse::NotFound().json("Not Found")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    /*
     * index: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/users
     * get: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/users/<id>
     * post: curl -i -X POST -H "Content-Type: application/json" -d '{"email":"xxx", "phone": "yyy"}' http://localhost:5000/users
     */

    cfg.service(
        web::resource("api/v1/students")
            .route(web::post().to(create))
            .route(web::get().to(index)),
    )
    .service(web::scope("api/v1/students").route("/{id}", web::get().to(get)));
}
