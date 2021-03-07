use crate::models;
use crate::pool::DbPool;
use crate::user::actions;
use actix_web::{delete, get, post, put, web, Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use uuid::Uuid;

impl Responder for models::User {
  type Error = Error;
  type Future = Ready<Result<HttpResponse, Error>>;

  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    let body = serde_json::to_string(&self).unwrap();

    // Create response and set content type
    ready(Ok(
      HttpResponse::Ok()
        .content_type("application/json")
        .body(body),
    ))
  }
}

// #[get("/show")]
// pub async fn show_users() -> impl Responder {
//   println!("show all users");
//   let mut vec: Vec<User> = Vec::new();
//   vec.push(User {
//     id: 1,
//     first_name: String::from("Paul"),
//     last_name: String::from("Simon"),
//     email: String::from("a.p"),
//     pin_code: 1234,
//   });
//   vec.push(User {
//     id: 12,
//     first_name: String::from("Jean"),
//     last_name: String::from("Valjean"),
//     email: String::from("j.m@laposte.net"),
//     pin_code: 12,
//   });
//   web::Json(vec)
// }

#[get("/show/{user_id}")]
async fn show_user(
  pool: web::Data<DbPool>,
  user_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
  let user_uid = user_uid.into_inner();
  let conn = pool.get().expect("couldn't get db connection from pool");

  // use web::block to offload blocking Diesel code without blocking server thread
  let user = web::block(move || actions::find_user_by_uid(user_uid, &conn))
    .await
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;

  if let Some(user) = user {
    Ok(HttpResponse::Ok().json(user))
  } else {
    let res = HttpResponse::NotFound().body(format!("No user found with uid: {}", user_uid));
    Ok(res)
  }
}

#[post("/")]
pub async fn add_user(
  pool: web::Data<DbPool>,
  form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
  println!("yooo");
  let conn = pool.get().expect("couldn't get db connection from pool");

  // use web::block to offload blocking Diesel code without blocking server thread
  let user = web::block(move || {
    actions::insert_new_user(&form.first_name, &form.last_name, &form.email, &conn)
  })
  .await
  .map_err(|e| {
    eprintln!("{}", e);
    HttpResponse::InternalServerError().finish()
  })?;

  Ok(HttpResponse::Ok().json(user))
}

// #[put("/")]
// pub async fn update() -> impl Responder {
//   println!("add new user");
//   User {
//     id: 12,
//     first_name: String::from("Jean"),
//     last_name: String::from("Valjean"),
//     email: String::from("j.m@laposte.net"),
//     pin_code: 12,
//   }
// }

// #[delete("/")]
// pub async fn delete() -> impl Responder {
//   println!("delete user");
//   User {
//     id: 12,
//     first_name: String::from("Jean"),
//     last_name: String::from("Valjean"),
//     email: String::from("j.m@laposte.net"),
//     pin_code: 12,
//   }
// }
