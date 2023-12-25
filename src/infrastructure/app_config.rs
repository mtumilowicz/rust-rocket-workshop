use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    orders_url: String,
}