use crate::models;
use crate::pool::DbPool;
use crate::user::actions;
use actix_web::{delete, get, post, web, Error, HttpRequest, HttpResponse, Responder};
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

#[get("")]
pub async fn show_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  println!("show all users");
  let conn = pool.get().expect("couldn't get db connection from pool");

  let users = web::block(move || actions::find_users(&conn))
    .await
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;
  Ok(HttpResponse::Ok().json(users))
}

#[get("/{user_id}")]
async fn show_user(
  pool: web::Data<DbPool>,
  user_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
  let user_uid = user_uid.into_inner();
  let conn = pool.get().expect("couldn't get db connection from pool");

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

#[post("")]
pub async fn upsert(
  pool: web::Data<DbPool>,
  form: web::Json<models::UserForm>,
) -> Result<HttpResponse, Error> {
  let conn = pool.get().expect("couldn't get db connection from pool");
  let user = web::block(move || match form.id {
    None => actions::insert_user(&form, &conn),
    Some(id) => actions::update_user(&id, &form, &conn),
  })
  .await
  .map_err(|e| {
    eprintln!("{}", e);
    HttpResponse::InternalServerError().finish()
  })?;

  Ok(HttpResponse::Ok().json(user))
}

#[delete("/{user_id}")]
pub async fn delete_user(
  pool: web::Data<DbPool>,
  user_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
  let user_uid = user_uid.into_inner();
  let conn = pool.get().expect("couldn't get db connection from pool");

  let users = web::block(move || actions::delete_user(user_uid, &conn))
    .await
    .map_err(|e| {
      eprintln!("{}", e);
      HttpResponse::InternalServerError().finish()
    })?;
  Ok(HttpResponse::Ok().json(users))
}
