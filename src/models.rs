use crate::schema::users;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub pin_code: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Struct for creating / Updating User
#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UserForm {
    pub id: Option<Uuid>,
    pub pin_code: Option<i16>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
