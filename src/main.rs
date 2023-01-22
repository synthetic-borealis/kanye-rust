#[macro_use]
extern crate rocket;

use std::net::Ipv4Addr;
use rocket::Config;
use rocket::routes;
use rocket::serde::json::Json;
use kanye_rust::{cors, quotes};

#[get("/text")]
fn text() -> &'static str {
    quotes::get_random_quote()
}

#[get("/")]
fn index() -> Json<quotes::Quote> {
    Json(quotes::Quote::new())
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    let config = Config {
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..Config::default()
    };
    rocket::custom(&config)
        .attach(cors::Cors)
        .mount("/", routes![index, text])
}
