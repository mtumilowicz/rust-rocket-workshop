use rocket::{get, launch, routes};
use rocket::serde::json::Json;
use serde_derive::Serialize;
use crate::customer::{Customer, CustomerId, CustomerService, NewCustomerCommand};
use crate::customer_config::CustomerInMemoryRepository;
use crate::id::{IdRepository, IdService};
use crate::id_config::{DeterministicRepository, UuidRepository};
use crate::gateway::{create_customer, get_customer};

mod id_config;
mod id;
mod customer;
mod customer_config;
mod b;
mod gateway;

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

    rocket::build()
        .manage(ggg)
        .mount("/api", routes![create_customer, get_customer])

}
