use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct GetUsersResponse {
    data: Vec<u32>,
}

async fn get_users() -> impl Responder {
    HttpResponse::Ok().json(GetUsersResponse { data: vec![32, 64] })
}

#[derive(Deserialize)]
struct GetUserPath {
    user_id: u32,
}

#[derive(Deserialize, Debug)]
struct GetUserQuery {
    offset: Option<u32>,
}

#[get("/{user_id}")]
async fn get_user(path: web::Path<GetUserPath>, query: web::Query<GetUserQuery>) -> impl Responder {
    if path.user_id == 0 {
        return HttpResponse::BadRequest().body("Bad Data");
    }

    HttpResponse::Ok().body(match query.offset {
        None => format!("Get User {}", path.user_id),
        Some(offset) => format!("Get User {} with offset {}", path.user_id, offset),
    })
}

#[derive(Deserialize)]
struct PostUserPayload {
    user_id: u32,
    username: String,
}
async fn post_user(info: web::Json<PostUserPayload>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Post User with user_id: {} and username: {}",
        info.user_id, info.username,
    ))
}

async fn put_user() -> impl Responder {
    HttpResponse::Ok().body("Put User")
}

async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("Delete User")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(get_users))
            .route(web::post().to(post_user))
            .route(web::put().to(put_user))
            .route(web::delete().to(delete_user)),
    )
    .service(web::scope("/users").service(get_user));
}
