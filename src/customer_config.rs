use std::collections::HashMap;
use std::sync::RwLock;
use crate::customer::{Customer, CustomerId, CustomerRepository, CustomerError};

pub struct CustomerInMemoryRepository {
    customers: HashMap<CustomerId, Customer>,
}

impl CustomerInMemoryRepository {
    pub fn new() -> Self {
        CustomerInMemoryRepository {
            customers: HashMap::new(),
        }
    }
}

impl CustomerRepository for CustomerInMemoryRepository {

    fn create(&mut self, customer: Customer) -> Result<Customer, CustomerError> {
        match self.get_by_id(customer.id()) {
            None => {
                self.customers.insert(customer.id().clone(), customer.clone());
                Ok(customer)
            }
            Some(_) =>
                Err(CustomerError::CustomerAlreadyExist)

        }

    }

    fn get_by_id(&self, customer_id: &CustomerId) -> Option<&Customer> {
        self.customers.get(customer_id)
    }
}