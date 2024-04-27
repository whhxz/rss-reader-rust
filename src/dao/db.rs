use diesel::connection::SimpleConnection;
use rocket::{fairing::AdHoc, Build, Rocket};
use rocket_sync_db_pools::database;
use std::fs::File;
use std::io::Read;

#[database("sqlite_rss_reader")]
pub struct DbConn(diesel::SqliteConnection);

async fn init_sql(rocket: Rocket<Build>) -> Rocket<Build> {
    DbConn::get_one(&rocket)
        .await
        .expect("database connection")
        .run(|conn| {
            let mut file = File::open("resource/init.sql").unwrap();
            let mut init_sql = String::new();
            let _ = file.read_to_string(&mut init_sql);
            conn.batch_execute(&init_sql)
        })
        .await
        .expect("init db error");
    rocket
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket
            .attach(DbConn::fairing())
            .attach(AdHoc::on_ignite("db init", init_sql))
    })
}
