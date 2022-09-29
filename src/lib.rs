use rocket::serde::Serialize;

pub mod quote_list;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Quote {
    quote: &'static str,
}

impl Quote {
    pub fn new() -> Self {
        Self {
            quote: quote_list::get_random_quote(),
        }
    }
}

impl Default for Quote {
    fn default() -> Self {
        Self::new()
    }
}
