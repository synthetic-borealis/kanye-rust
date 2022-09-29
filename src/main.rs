#[macro_use]
extern crate rocket;

use kanye_rust::quote_list;
use kanye_rust::Quote;
use rocket::serde::json::Json;

#[get("/text")]
fn text() -> &'static str {
    quote_list::get_random_quote()
}

#[get("/")]
fn index() -> Json<Quote> {
    Json(Quote::new())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, text])
}
