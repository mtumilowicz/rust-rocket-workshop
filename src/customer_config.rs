use std::collections::HashMap;
use std::sync::RwLock;
use crate::customer::{Customer, CustomerId, CustomerRepository, CustomerError};

pub struct CustomerInMemoryRepository {
    requests: RwLock<HashMap<CustomerId, Customer>>,
}

impl CustomerInMemoryRepository {
    pub fn new() -> Self {
        CustomerInMemoryRepository {
            requests: RwLock::new(HashMap::new()),
        }
    }
}

impl CustomerRepository for CustomerInMemoryRepository {

    fn create(&self, customer: Customer) -> Result<Customer, CustomerError> {
        match self.get_by_id(customer.id()) {
            None => {
                self.requests.write().unwrap().insert(customer.id().clone(), customer.clone());
                Ok(customer.clone())
            }
            Some(_) =>
                Err(CustomerError::CustomerAlreadyExist)

        }

    }

    fn get_by_id(&self, customer_id: &CustomerId) -> Option<Customer> {
        self.requests.read().unwrap().get(customer_id).cloned()
    }
}