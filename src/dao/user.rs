use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{deserialize::Queryable, prelude::Insertable};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = user)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
    pub last_login_time: Option<NaiveDateTime>,
}

table! {
    user (id) {
        id -> Nullable<Integer>,
        username -> Text,
        password -> Text,
        is_admin -> Bool,
        last_login_time -> Nullable<Timestamp>
    }
}
type Result<T, E = diesel::result::Error> = std::result::Result<T, E>;

pub fn select_by_name(conn: &mut diesel::SqliteConnection, username: &str) -> Result<Vec<User>> {
    let users: Result<Vec<User>> = user::table.filter(user::username.eq(username)).load(conn);
    // user::table.select(user::id).filter(user::username.eq(username));
    // user::table.filter(user::username.eq(username)).first();
    users
}

pub fn insert(conn: &mut diesel::SqliteConnection, user: &User) -> Result<usize> {
    diesel::insert_into(user::table).values(user).execute(conn)
}
