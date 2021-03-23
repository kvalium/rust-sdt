use crate::schema::users;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // pub deleted_at: Option<NaiveDateTime>,
}

// Struct for creating Book
#[derive(Debug, PartialEq, Clone, Insertable, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UserForm {
    pub id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NewUser {
//     pub id: Option<Uuid>,
//     pub first_name: String,
//     pub last_name: String,
//     pub email: String,
// }
