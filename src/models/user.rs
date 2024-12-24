use diesel::{Queryable, Selectable, Insertable};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}

#[derive(Debug, Insertable, Validate, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    #[validate(length(min = 3, max = 255))]
    pub username: String,
    #[validate(length(min = 8, max = 255))]
    pub password: String,
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}