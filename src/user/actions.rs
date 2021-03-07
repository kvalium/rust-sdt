use diesel::prelude::*;
use uuid::Uuid;

use crate::models;

pub fn find_user_by_uid(
  uid: Uuid,
  conn: &PgConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
  use crate::schema::users::dsl::*;

  let user = users
    .filter(id.eq(uid))
    .first::<models::User>(conn)
    .optional()?;
  Ok(user)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_user(
  // prevent collision with `name` column imported inside the function
  firstn: &str,
  lastn: &str,
  mail: &str,
  conn: &PgConnection,
) -> Result<models::User, diesel::result::Error> {
  // It is common when using Diesel with Actix web to import schema-related
  // modules inside a function's scope (rather than the normal module's scope)
  // to prevent import collisions and namespace pollution.
  use crate::schema::users::dsl::*;

  let new_user = models::User {
    id: Uuid::new_v4(),
    first_name: firstn.to_owned(),
    last_name: lastn.to_owned(),
    email: mail.to_owned(),
  };

  diesel::insert_into(users).values(&new_user).execute(conn)?;

  Ok(new_user)
}
