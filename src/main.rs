use std::fs::File;
use std::io::Read;

use rocket::fairing::AdHoc;
use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use rocket_dyn_templates::{context, Template};
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::rusqlite::{self, params};
use rss_reader_rust::model::user::User;
#[macro_use]
extern crate rocket;

#[database("sqlite_rss_reader")]
pub struct DbConn(rusqlite::Connection);

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

type Result<T, E = Debug<rusqlite::Error>> = std::result::Result<T, E>;
#[get("/db/list")]
async fn db_list(db: DbConn) -> Result<Json<Vec<i64>>> {
    let ids = db
        .run(|conn| {
            conn.prepare("SELECT id FROM user")?
                .query_map(params![], |row| {
                    let a = row.get(0);
                    a
                })?
                .collect::<Result<Vec<i64>, _>>()
        })
        .await?;

    Ok(Json(ids))
}

#[post("/db/create", data = "<data>")]
async fn db_create(db: DbConn, data: Json<User>) -> Option<Json<User>> {
    let item = data.clone();

    let insert = db
        .run(move |conn| {
            conn.execute(
                "INSERT INTO user (username, password, is_admin) VALUES (?1, ?2, ?3)",
                params![item.username, item.password, item.is_admin],
            )
        })
        .await;
    let item = data.clone();
    let user = match insert {
        Ok(_) => {
            let user = db
                .run(move |conn| {
                    conn.query_row(
                        "select id, username, password, is_admin from user where username = ?1",
                        params![item.username],
                        |r| {
                            Ok(User {
                                id: Some(r.get(0)?),
                                username: r.get(1)?,
                                password: r.get(2)?,
                                is_admin: r.get(3)?,
                                last_login_time: None,
                            })
                        },
                    )
                })
                .await
                .ok()?;
            Some(user)
        }
        Err(e) => {
            println!("{:?}", e);
            None
        }
    };
    user.map(|x| Json(x))
}
async fn init_db(rocket: Rocket<Build>) -> Rocket<Build> {
    DbConn::get_one(&rocket)
        .await
        .expect("database mounted")
        .run(|conn| {
            let mut file = File::open("resource/init.sql").unwrap();
            let mut init_sql = String::new();
            let _ = file.read_to_string(&mut init_sql);
            conn.execute_batch(&init_sql)
        })
        .await
        .expect("can init rusqlite DB");
    rocket
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build()
        .mount("/", routes![index, json, template, db_list, db_create])
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("SQLx Migrations", init_db))
        .attach(Template::fairing());
    rocket
}
