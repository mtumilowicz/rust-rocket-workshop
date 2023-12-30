use rocket::{Build, Rocket, routes};
use crate::domain::customer::{CustomerService};
use crate::infrastructure::customer_config::CustomerInMemoryRepository;
use crate::domain::id::{IdService};
use crate::infrastructure::id_config::{UuidRepository};
use crate::gateway::customer::{create_customer, get_customer};
use std::sync::Arc;

pub async fn server() -> Rocket<Build> {
    let id_service = Arc::new(IdService::new(UuidRepository));
    let customer_service = CustomerService::new(id_service, CustomerInMemoryRepository::new());
    rocket::build()
        .manage(customer_service)
        .mount("/api", routes![create_customer, get_customer])
}