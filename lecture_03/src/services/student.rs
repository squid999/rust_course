use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::db::{
    models::{Class, StuClass, Student},
    DbPool,
};

#[derive(Serialize, Deserialize)]
pub struct StudentForm {
    id: Option<i32>,
    sid: String,
    name: Option<String>,
    class: Option<String>,
}

pub fn create(stu_form: web::Json<StudentForm>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();

    match Student::create(&stu_form.sid, stu_form.name.as_deref(), &conn) {
        Some(stu) => HttpResponse::Ok().json(stu),
        _ => HttpResponse::InternalServerError().json("Could not create student object"),
    }
    // HttpResponse::Ok().json("success")
}

pub fn index(pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();

    HttpResponse::Ok().json(Student::list(&conn))
    // HttpResponse::Ok().json("[]")
}

pub fn get(sid: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();

    match Student::by_sid(&sid, &conn) {
        Some(stu) => match StuClass::by_sid(&sid, &conn) {
            Some(entry) => match Class::by_clsid(&entry.clsid, &conn) {
                Some(cls) => {
                    let scf = StudentForm {
                        id: stu.id,
                        sid: sid.to_string(),
                        name: stu.name,
                        class: cls.name,
                    };
                    HttpResponse::Ok().json(scf)
                }
                None => HttpResponse::Ok().json(stu),
            },
            None => HttpResponse::Ok().json(stu),
        },
        _ => HttpResponse::NotFound().json("Not Found"),
    }
    // HttpResponse::NotFound().json("Not Found")
}

pub fn update(stu_form: web::Json<StudentForm>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();

    match stu_form.name.as_ref() {
        Some(name_) => {
            if name_.len() == 0 {
                return HttpResponse::BadRequest().json("name field needed");
            }
        }

        None => {
            return HttpResponse::BadRequest().json("name field needed");
        }
    };
    match Student::update(&stu_form.sid, stu_form.name.as_deref(), &conn) {
        Ok(_) => match Student::by_sid(&stu_form.sid, &conn) {
            Some(stu) => HttpResponse::Ok().json(stu),
            _ => HttpResponse::Ok().json(""),
        },
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

pub fn del(sid: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();

    match Student::remove(&sid, &conn) {
        Ok(_) => match StuClass::remove(&sid, &conn) {
            Ok(_) => HttpResponse::Ok().json("success"),
            Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
        },
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    /*
     * index: curl -i -X GET -H "Content-Type: application/json" http://localhost:8111/students
     * get: curl -i -X GET -H "Content-Type: application/json" http://localhost:8111/students/<sid>
     * post: curl -i -X POST -H "Content-Type: application/json" -d '{"email":"xxx", "phone": "yyy"}' http://localhost:8111/students
     */

    cfg.service(
        web::resource("api/v1/students")
            .route(web::post().to(create))
            .route(web::get().to(index))
            .route(web::put().to(update)),
    )
    .service(
        web::resource("api/v1/students/{sid}")
            .route(web::get().to(get))
            .route(web::delete().to(del)),
    );
}
