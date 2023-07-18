use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::db::DbPool;

#[derive(Serialize, Deserialize)]
pub struct ClubForm {
    name: Option<String>,
    description: Option<String>,
}

pub fn create(club_form: web::Json<ClubForm>, pool: web::Data<DbPool>) -> HttpResponse {
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

pub fn get(club_name: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    // let conn = pool.get().unwrap();

    // match Student::by_id(&id, &conn) {
    //     Some(stu) => HttpResponse::Ok().json(stu),
    //     _ => HttpResponse::NotFound().json("Not Found"),
    // }
    // println!("{:?}", club_name);
    // HttpResponse::NotFound().json("Not Found")
    // let out = format!("{{\"club\": \"{}\"}}", club_name);
    // println!("{}", out);

    let club = ClubForm {
        // name: &club_name,
        name: Some(club_name.to_string()),
        description: Some("wonderfull club".to_string()),
    };
    HttpResponse::Ok().json(&club)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    /*
     * index: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/users
     * get: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/users/<id>
     * post: curl -i -X POST -H "Content-Type: application/json" -d '{"email":"xxx", "phone": "yyy"}' http://localhost:5000/users
     */

    cfg.service(
        web::resource("api/v1/clubs")
            .route(web::post().to(create))
            .route(web::get().to(index)),
    )
    .service(web::scope("api/v1/clubs").route("/{club_name}", web::get().to(get)));
}
