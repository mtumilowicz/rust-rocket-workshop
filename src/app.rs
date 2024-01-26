use rocket::{Build, Rocket, routes};
use crate::domain::customer::{CustomerService};
use crate::gateway::customer::{create_customer, get_customer};
use std::sync::Arc;

pub fn server(
    customer_service: Arc<CustomerService>
) -> Rocket<Build> {
    rocket::build()
        .manage(customer_service)
        .mount("/api", routes![create_customer, get_customer])
}