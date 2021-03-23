use crate::models;
use crate::models::UserForm;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

pub fn find_users(conn: &PgConnection) -> Result<Vec<models::User>, diesel::result::Error> {
  users.load::<models::User>(conn)
}

pub fn find_user_by_uid(
  uid: Uuid,
  conn: &PgConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
  let user = users
    .filter(id.eq(uid))
    .first::<models::User>(conn)
    .optional()?;
  Ok(user)
}

pub fn delete_user(uid: Uuid, conn: &PgConnection) -> Result<usize, diesel::result::Error> {
  diesel::delete(users.filter(id.eq(uid))).execute(conn)
}

pub fn insert_user(
  user_form: &UserForm,
  conn: &PgConnection,
) -> Result<Vec<models::User>, diesel::result::Error> {
  let results = diesel::insert_into(users)
    .values(user_form)
    .get_results::<models::User>(conn)?;
  Ok(results)
}

pub fn update_user(
  uid: &Uuid,
  user_form: &UserForm,
  conn: &PgConnection,
) -> Result<Vec<models::User>, diesel::result::Error> {
  let results = diesel::update(users.filter(id.eq(uid)))
    .set(user_form)
    .get_results::<models::User>(conn)?;
  Ok(results)
}
