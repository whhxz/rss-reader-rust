use chrono::Utc;
use rocket::serde::json::Json;
use rocket_dyn_templates::{context, Template};
use rss_reader_rust::dao::{self, user::User};
#[macro_use]
extern crate rocket;
extern crate rocket_sync_db_pools;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/json")]
fn json() -> Json<Vec<i32>> {
    Json(vec![1, 2, 3])
}
#[get("/temp/<name>")]
fn template(name: &str) -> Template {
    Template::render(
        "hello",
        context! {
            name: name
        },
    )
}

#[get("/user")]
async fn user(db: dao::db::DbConn) -> Json<Vec<User>> {
    let user = User {
        id: None,
        username: String::from("123"),
        password: String::from("value"),
        is_admin: true,
        last_login_time: Some(Utc::now().naive_local()),
    };
    let _ = db.run(move |conn| dao::user::insert(conn, &user)).await;
    let data = db
        .run(move |conn| dao::user::select_by_name(conn, "123"))
        .await;
    Json(data.unwrap())
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build()
        .mount("/", routes![index, json, template, user])
        .attach(dao::db::stage())
        .attach(Template::fairing());
    rocket
}
