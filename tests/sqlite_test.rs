// use std::{env::current_dir, fs::File, io::Read};

// use chrono::Utc;
// use rocket_db_pools::sqlx::Connection;
// use rss_reader_rust::model::user::User;

// #[test]
// fn demo1() -> Result<(), Box<dyn std::error::Error>> {
//     let conn = Connection::open_in_memory()?;

//     let mut path = current_dir()?;
//     path.push("resource");
//     path.push("init.sql");

//     let mut file = File::open(path)?;
//     let mut init_sql = String::new();
//     file.read_to_string(&mut init_sql)?;

//     let _ = conn.execute_batch(&init_sql);

//     let user = User {
//         id: 0,
//         username: String::from("admin"),
//         password: String::from("123"),
//         is_admin: true,
//         last_login_time: Some(Utc::now()),
//     };
//     let res = conn.execute(
//         "insert into user (username, password, is_admin, last_login_time) values (?1,?2,?3,?4)",
//         (
//             &user.username,
//             &user.password,
//             &user.is_admin,
//             &user.last_login_time,
//         ),
//     )?;
//     assert_eq!(1, res);

//     let mut stmt =
//         conn.prepare("select id,username, password, is_admin, last_login_time from user")?;
//     let user_iter = stmt.query_map([], |row| {
//         Ok(User {
//             id: row.get(0)?,
//             username: row.get(1)?,
//             password: row.get(2)?,
//             is_admin: row.get(3)?,
//             last_login_time: row.get(4)?,
//         })
//     })?;
//     for user in user_iter {
//         println!("Found user {:?}", user?);
//     }
//     Ok(())
// }
