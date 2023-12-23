use rocket::{get, launch, routes};
use rocket::serde::json::Json;
use serde_derive::Serialize;
use crate::customer::{Customer, CustomerId, CustomerService, NewCustomerCommand};
use crate::customer_config::CustomerInMemoryRepository;
use crate::id::{IdRepository, IdService};
use crate::id_config::{DeterministicRepository, UuidRepository};

mod id_config;
mod id;
mod customer;
mod customer_config;

#[derive(Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
struct CustomerApiOutput {
    id: String,
    name: String,
    lock: bool
}

impl From<&Customer> for CustomerApiOutput {
    fn from(value: &Customer) -> Self {
        CustomerApiOutput {
            id: value.id().clone().0,
            name: value.name().to_string(),
            lock: value.locked()
        }
    }
}

#[get("/get_request/<name>")]
pub async fn get_request_handler(
    name: &str,
    service: &rocket::State<CustomerService>,
) -> Option<Json<CustomerApiOutput>> {
    // Retrieve the HelloRequest from the service by name
    let request = service.get_by_id(&CustomerId::new(name)).await;

    request.map(|r| r.into()).map(Json)
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
        .mount("/api", routes![get_request_handler])

}
