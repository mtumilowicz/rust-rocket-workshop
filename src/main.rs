use std::sync::Arc;
use rocket::{launch};
use rust_rocket_workshop::app::server;
use rust_rocket_workshop::domain::customer::CustomerService;
use rust_rocket_workshop::domain::id::IdService;
use rust_rocket_workshop::infrastructure::customer_config::CustomerInMemoryRepository;
use rust_rocket_workshop::infrastructure::id_config::UuidRepository;

#[launch]
fn rocket() -> _ {
    let id_service = Arc::new(IdService::new(UuidRepository));
    let customer_service = Arc::new(CustomerService::new(id_service, CustomerInMemoryRepository::new()));
    server(
        customer_service
    )
}
