use std::collections::HashMap;
use std::sync::RwLock;
use crate::domain::customer::{Customer, CustomerId, CustomerRepository, CustomerError};

pub struct CustomerInMemoryRepository {
    customers: RwLock<HashMap<CustomerId, Customer>>,
}

impl CustomerInMemoryRepository {
    pub fn new() -> Self {
        CustomerInMemoryRepository {
            customers: RwLock::new(HashMap::new()),
        }
    }
}

impl CustomerRepository for CustomerInMemoryRepository {

    fn create(&self, customer: Customer) -> Result<Customer, CustomerError> {
        let customer_id = customer.id();
        match self.get_by_id(customer_id) {
            None => {
                self.customers.write().unwrap().insert(customer_id.clone(), customer.clone());
                Ok(customer)
            }
            Some(_) =>
                Err(CustomerError::CustomerAlreadyExist(customer_id.clone()))

        }

    }

    fn get_by_id(&self, customer_id: &CustomerId) -> Option<Customer> {
        self.customers.read().unwrap().get(customer_id).cloned()
    }
}