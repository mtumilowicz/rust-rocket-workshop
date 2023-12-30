use std::sync::Arc;
use crate::domain::id::IdService;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct CustomerId(String);

impl CustomerId {
    pub fn new(value: String) -> Self {
        CustomerId(value)
    }

    pub fn raw(&self) -> String {
        self.0.to_string()
    }
}

impl From<String> for CustomerId {
    fn from(value: String) -> Self {
        CustomerId::new(value)
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Customer {
    id: CustomerId,
    name: String,
    locked: bool,
}

impl Customer {
    pub fn id(&self) -> &CustomerId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn locked(&self) -> bool {
        self.locked
    }
}

pub struct NewCustomerCommand {
    name: String,
    locked: bool,
}

impl NewCustomerCommand {

    pub fn new(name: String, locked: bool) -> Self {
        NewCustomerCommand { name, locked }
    }
    pub fn to_customer(self, id: CustomerId) -> Customer {
        Customer {
            id,
            name: self.name,
            locked: self.locked,
        }
    }
}

pub struct CustomerService {
    id_service: Arc<IdService>,
    repository: Box<dyn CustomerRepository + Send + Sync>,
}

impl CustomerService {

    pub fn new<T: CustomerRepository + Send + Sync + 'static>(id_service: Arc<IdService>, repository: T) -> CustomerService {
        CustomerService {
            id_service: id_service,
            repository: Box::new(repository)
        }
    }
    pub async fn create(&self, command: NewCustomerCommand) -> Result<Customer, CustomerError> {
        let id = self.id_service.generate().await;
        let customer = command.to_customer(CustomerId(id));
        self.repository.create(customer)
    }

    pub async fn get_by_id(&self, id: &CustomerId) -> Option<Customer> {
        self.repository.get_by_id(id)
    }
}

#[derive(Debug)]
pub enum CustomerError {
    CustomerAlreadyExist,
}

pub trait CustomerRepository {
    fn create(&self, customer: Customer) -> Result<Customer, CustomerError>;
    fn get_by_id(&self, customer_id: &CustomerId) -> Option<Customer>;

}

#[cfg(test)]
mod tests {
    use rocket::async_test;
    use crate::infrastructure::customer_config::CustomerInMemoryRepository;
    use crate::infrastructure::id_config::DeterministicRepository;
    use super::*;

    #[async_test]
    async fn test_create_and_get_customer() {
        let id_service = Arc::new(IdService::new(DeterministicRepository::new()));
        let customer_repository = CustomerInMemoryRepository::new();
        let customer_service = CustomerService::new(id_service.clone(), customer_repository);

        let new_customer_command = NewCustomerCommand::new("John Doe".to_string(), false);

        let created_customer = customer_service.create(new_customer_command).await.unwrap();

        let retrieved_customer = customer_service.get_by_id(&created_customer.id()).await.unwrap();

        assert_eq!(retrieved_customer, created_customer);
    }

    #[async_test]
    async fn test_get_nonexistent_customer() {
        let id_service = Arc::new(IdService::new(DeterministicRepository::new()));
        let customer_repository = CustomerInMemoryRepository::new();
        let customer_service = CustomerService::new(id_service.clone(), customer_repository);

        let customer_id = CustomerId("nonexistent_id".to_string());

        let result = customer_service.get_by_id(&customer_id).await;

        assert!(result.is_none());
    }
}