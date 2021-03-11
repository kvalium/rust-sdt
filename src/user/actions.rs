use crate::schema::users::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

use crate::models;

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

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_user(
  // prevent collision with `name` column imported inside the function
  firstn: &str,
  lastn: &str,
  mail: &str,
  conn: &PgConnection,
) -> Result<models::User, diesel::result::Error> {
  let new_user = models::User {
    id: Uuid::new_v4(),
    first_name: firstn.to_owned(),
    last_name: lastn.to_owned(),
    email: mail.to_owned(),
  };

  diesel::insert_into(users).values(&new_user).execute(conn)?;

  Ok(new_user)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn update_user(
  uid: &Uuid,
  firstn: &str,
  lastn: &str,
  mail: &str,
  conn: &PgConnection,
) -> Result<models::User, diesel::result::Error> {
  let updated_row = diesel::update(users.filter(id.eq(uid)))
    .set((
      first_name.eq(firstn.to_owned()),
      last_name.eq(lastn.to_owned()),
      email.eq(mail.to_owned()),
    ))
    .get_result(conn)?;

  Ok(updated_row)
}
