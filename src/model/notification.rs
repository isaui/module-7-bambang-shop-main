use rocket::serde::{Deserialize, Serialize};
use rocket::log;
use rocket::serde::json::to_string;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde()]
pub struct Notification{
    pub product_title: String,
    pub product_type: String,
    pub product_url: String,
    pub subscriber_name: String,
    pub status: String,
}