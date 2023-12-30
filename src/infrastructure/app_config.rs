use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct AppConfig {
    orders_url: String,
}

pub async fn load_config() -> rocket::figment::error::Result<AppConfig> {
    rocket::Config::figment().extract::<AppConfig>()
}

#[cfg(test)]
mod tests {
    use std::env;
    use rocket::async_test;
    use super::*;

    #[async_test]
    async fn test_load_config_with_development_profile() {
        env::set_var("ROCKET_PROFILE", "development");

        let config = load_config().await.unwrap();

        assert_eq!(config.orders_url, "api/orders");
    }
}