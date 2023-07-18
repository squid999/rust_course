use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::db::{models::StuClass, DbPool};

#[derive(Serialize, Deserialize)]
pub struct StuClassForm {
    stu_id: String,
    class_id: String,
    comment: Option<String>,
}

pub async fn create(sc_form: web::Json<StuClassForm>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();

    match StuClass::create(
        &sc_form.stu_id,
        &sc_form.class_id,
        sc_form.comment.as_deref(),
        &conn,
    ) {
        Some(entry) => HttpResponse::Ok().json(entry),
        _ => HttpResponse::InternalServerError().json("Could not register student for class"),
    }
}

pub async fn index(pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();

    HttpResponse::Ok().json(StuClass::list(&conn))
}

pub async fn get(sid: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();

    match StuClass::by_sid(&sid, &conn) {
        Some(entry) => HttpResponse::Ok().json(entry),
        _ => HttpResponse::NotFound().json("Not Found"),
    }
}

pub fn del(sid: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();

    match StuClass::remove(&sid, &conn) {
        Ok(_) => HttpResponse::Ok().json("success"),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    /*
     * index: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/stucls
     * get: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/stucls/<clsid>
     * delete: curl -i -X DELETE -H "Content-Type: application/json" http://localhost:5000/stucls/<clsid>
     * post: curl -i -X POST -H "Content-Type: application/json" -d '{"email":"xxx", "phone": "yyy"}' http://localhost:5000/classes
     */

    cfg.service(
        web::resource("api/v1/stucls")
            .route(web::post().to(create))
            .route(web::get().to(index)),
    )
    .service(
        web::resource("api/v1/stucls/{sid}")
            .route(web::get().to(get))
            .route(web::delete().to(del)),
    );
}
