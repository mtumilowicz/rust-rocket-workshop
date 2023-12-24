use std::thread;
use crate::customer::{CustomerId, CustomerRepository, NewCustomerCommand};
use crate::customer_config::CustomerInMemoryRepository;

fn main() {
    // Create a repository
    let mut repository = CustomerInMemoryRepository::new();

    // Create two threads
    let thread1 = thread::spawn(move || {
        let customer1 = NewCustomerCommand::new(
            String::from("John Doe3"),
            false,
        );
        let result = repository.create(customer1.to_customer(CustomerId::new(String::from("a"))));
        println!("{:?}", result);
    });

    // let thread2 = thread::spawn(move || {
    //     let customer2 = NewCustomerCommand::new(
    //         String::from("John Doe3"),
    //         false,
    //     );
    //     let result = repository.create(customer2.to_customer(CustomerId::new("b")));
    //     println!("{:?}", result);
    // });

    // Wait for both threads to finish
    thread1.join().unwrap();
    // thread2.join().unwrap();
}
