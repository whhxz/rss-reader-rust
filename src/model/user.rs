#![allow(dead_code)]
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
    pub last_login_time: Option<DateTime<Utc>>,
}

pub struct UserSession {
    pub id: Option<i32>,
    pub user_id: i32,
    pub token: String,
    pub create_time: DateTime<Utc>,
    pub ip: String,
}
