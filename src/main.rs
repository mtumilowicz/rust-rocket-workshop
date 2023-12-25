use rocket::{launch, routes};
use serde_derive::Serialize;
use crate::domain::customer::{Customer, CustomerService, NewCustomerCommand};
use infrastructure::customer_config::CustomerInMemoryRepository;
use domain::id::{IdRepository, IdService};
use crate::infrastructure::id_config::{DeterministicRepository, UuidRepository};
use crate::gateway::customer::{create_customer, get_customer};
use crate::infrastructure::app_config::AppConfig;

mod b;
mod domain;
mod gateway;
mod infrastructure;

#[derive(Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
struct CustomerApiOutput {
    id: String,
    name: String,
    lock: bool
}

impl From<Customer> for CustomerApiOutput {
    fn from(value: Customer) -> Self {
        CustomerApiOutput {
            id: value.id().raw(),
            name: value.name().to_string(),
            lock: value.locked()
        }
    }
}

#[launch]
async fn rocket() -> _ {
    let a = UuidRepository;
    let b = a.get();
    let c = a.get();
    let cc = IdService::new(a);
    let d = cc.generate().await;
    println!("{} {} {}", b, c, d);
    println!("-----------------");

    let aa = DeterministicRepository::new();
    let ccc = IdService::new(aa);
    let dd = ccc.generate().await;
    let ddd = ccc.generate().await;
    let dddd = ccc.generate().await;
    println!("{} {} {}", dd, ddd, dddd);

    let gg = CustomerInMemoryRepository::new();
    let mut ggg = CustomerService::new(cc, gg);
    let hh = ggg.create(NewCustomerCommand::new(
        String::from("John Doe"),
              false,
    )).await;
    let hhh = ggg.create(NewCustomerCommand::new(
        String::from("John Doe2"),
        false,
    )).await;
    let hhhh = ggg.create(NewCustomerCommand::new(
        String::from("John Doe3"),
        false,
    )).await;
    println!("{:?} {:?} {:?}", hh, hhh, hhhh);
    println!("Hello, world!");

    // env var ROCKET_PROFILE=development
    let config2 = rocket::Config::figment().extract::<AppConfig>();
    // Print out our settings
    println!("c------------------------{:?}", config2);

    rocket::build()
        .manage(ggg)
        .mount("/api", routes![create_customer, get_customer])

}
