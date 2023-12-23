use crate::customer::{CustomerService, NewCustomerCommand};
use crate::customer_config::CustomerInMemoryRepository;
use crate::id::{IdRepository, IdService};
use crate::id_config::{DeterministicRepository, UuidRepository};

mod id_config;
mod id;
mod customer;
mod customer_config;

#[tokio::main]
async fn main() {
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
    let ggg = CustomerService::new(cc, gg);
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
}
