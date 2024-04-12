use rocket::response::status::Created;
use rocket::serde::json::Json;

use bambangshop::Result;
use crate::model::subscriber::Subscriber;
use crate::repository::subscriber::SubscriberRepository;
use  crate::service::notification::NotificationService;
use crate::service::product::ProductService;

#[post("/subscribe/<product_type>", data = "<subscriber>")]
pub fn subscribe(product_type: &str, subscriber: Subscriber) -> Result<Subscriber>{
    let product_type_upper: String = product_type.to_uppercase();
    let product_type_str: &str = product_type_upper.as_str();
    let subscriber_result: Subscriber =
        SubscriberRepository::add(product_type_str, subscriber);
    return Ok(subscriber_result);
}

