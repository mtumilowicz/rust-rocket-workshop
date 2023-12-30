use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct AppConfig {
    orders_url: String,
}

// env var ROCKET_PROFILE=development
pub fn load_config() -> rocket::figment::error::Result<AppConfig> {
    rocket::Config::figment().extract::<AppConfig>()
}