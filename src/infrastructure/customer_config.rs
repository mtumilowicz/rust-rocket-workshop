use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use rocket::async_trait;
use rocket::tokio::task::spawn_blocking;
use crate::domain::customer::{Customer, CustomerId, CustomerRepository, CustomerError};

pub struct CustomerInMemoryRepository {
    customers: Arc<RwLock<HashMap<CustomerId, Customer>>>,
}

impl CustomerInMemoryRepository {
    pub fn new() -> Self {
        CustomerInMemoryRepository {
            customers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl CustomerRepository for CustomerInMemoryRepository {

     async fn create(&self, customer: Customer) -> Result<Customer, CustomerError> {
        let customer_id = customer.id();
        match self.get_by_id(customer_id).await {
            None => {
                let c_id = customer_id.clone();
                let c = customer.clone();
                let customers = self.customers.clone();
                let _ = spawn_blocking(move ||
                    customers.write().unwrap().insert(c_id, c)
                ).await;
                Ok(customer)
            }
            Some(_) =>
                Err(CustomerError::CustomerAlreadyExist(customer_id.clone()))

        }

    }

     async fn get_by_id(&self, customer_id: &CustomerId) -> Option<Customer> {
        self.customers.read().unwrap().get(customer_id).cloned()
    }
}