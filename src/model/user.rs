#![allow(dead_code)]
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
    pub last_login_time: Option<DateTime<Utc>>,
}

pub struct UserSession {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub create_time: DateTime<Utc>,
    pub ip: String,
}
