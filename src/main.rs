#[macro_use]
extern crate rocket;

use kanye_rust::{cors, quotes};
use rocket::routes;
use rocket::serde::json::Json;

#[get("/text")]
fn text() -> &'static str {
    quotes::get_random_quote()
}

#[get("/")]
fn index() -> Json<quotes::Quote> {
    Json(quotes::Quote::new())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(cors::Cors)
        .mount("/", routes![index, text])
}
