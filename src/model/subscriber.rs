use rocket::serde::{Deserialize, Serialize};
use  rocket::log;
use rocket::serde::json::to_string;
use bambangshop::REQWEST_CLIENT;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub  struct Subscriber {
    pub url: String,
    pub name: String,
}