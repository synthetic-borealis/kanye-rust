#[macro_use]
extern crate rocket;

use crate::quotes::*;
use rocket::serde::json::Json;

mod quotes;

#[get("/text")]
fn text() -> &'static str {
    get_random_quote()
}

#[get("/")]
fn index() -> Json<Quote> {
    Json(Quote::new())
}

#[shuttle_service::main]
async fn rocket() -> shuttle_service::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index, text]);

    Ok(rocket)
}
