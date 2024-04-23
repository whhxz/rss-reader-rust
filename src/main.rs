use rocket::serde::json::Json;
use rocket::Config;
use rocket_dyn_templates::{context, Template};
#[macro_use]
extern crate rocket;

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
#[launch]
fn rocket() -> _ {
    let config = Config::figment()
        .merge(("port", 6789))
        .merge(("host", "localhost"));
    rocket::build()
        .configure(config)
        .mount("/", routes![index, json, template])
        .attach(Template::fairing())
}
