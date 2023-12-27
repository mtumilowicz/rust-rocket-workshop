use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use std::thread;
use crate::domain::customer::{CustomerId, CustomerRepository, NewCustomerCommand};
use crate::infrastructure::customer_config::CustomerInMemoryRepository;

fn test<T>(data: Arc<RwLock<T>>)
    where
        T: 'static + Send + Sync + Debug,
{
    // Spawn a worker thread
    let handle = thread::spawn(move || {
        // Access the data inside the RwLock
        let read_guard = data.read().unwrap();
        println!("Worker Thread: Value inside RwLock: {:?}", *read_guard);
    });

    // Wait for the worker thread to finish
    handle.join().unwrap();
}

struct MyStruct {
    data: String,
}

// fn create_struct() -> &'static MyStruct {
//     let my_struct = MyStruct {
//         data: String::from("Hello, Rust!"),
//     };
//
//     &my_struct // Error: Cannot return reference to temporary value
// }


fn main() {

    // {
    //     let t = 3;
    //     let r = &t;
    //     let a = Arc::new(RwLock::new(r));
    //     test(a);
    // }

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
