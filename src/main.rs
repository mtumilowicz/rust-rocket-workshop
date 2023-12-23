use crate::id::{IdRepository, IdService};
use crate::id_config::{DeterministicRepository, UuidRepository};

mod id_config;
mod id;

fn main() {
    let a = UuidRepository;
    let b = a.get();
    let c = a.get();
    let cc = IdService::new(a);
    let d = cc.generate();
    println!("{} {} {}", b, c, d);
    println!("-----------------");
    let aa = DeterministicRepository::new();
    let ccc = IdService::new(aa);
    let dd = ccc.generate();
    let ddd = ccc.generate();
    let dddd = ccc.generate();
    println!("{} {} {}", dd, ddd, dddd);
    println!("Hello, world!");
}
