use crate::schema::users;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    // pub created_at: Option<NaiveDateTime>,
    // pub deleted_at: Option<NaiveDateTime>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
