use std::fmt;
use std::sync::Arc;
use thiserror::Error;
use crate::domain::id::IdService;
#[cfg(test)]
use mockall::*;
use rocket::async_trait;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct CustomerId(String);

impl fmt::Display for CustomerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
        self.repository.create(customer).await
    }

    pub async fn get_by_id(&self, id: &CustomerId) -> Option<Customer> {
        self.repository.get_by_id(id).await
    }
}

#[derive(Debug, Error)]
pub enum CustomerError {
    #[error("customer with id {0} already exists")]
    CustomerAlreadyExist(CustomerId),
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CustomerRepository {
    async fn create(&self, customer: Customer) -> Result<Customer, CustomerError>;
    async fn get_by_id(&self, customer_id: &CustomerId) -> Option<Customer>;

}

#[cfg(test)]
mod tests {
    use rocket::async_test;
    use crate::infrastructure::customer_config::CustomerInMemoryRepository;
    use crate::infrastructure::id_config::UuidRepository;
    use super::*;

    #[async_test]
    async fn test_create_and_get_customer() -> Result<(), Box<dyn std::error::Error>> {
        let customer_service = crate_customer_service();

        let new_customer_command = NewCustomerCommand::new("John Doe".to_string(), false);

        let created_customer = customer_service.create(new_customer_command).await?;

        let retrieved_customer = customer_service.get_by_id(&created_customer.id()).await.ok_or("customer not found")?;

        assert_eq!(retrieved_customer, created_customer);

        Ok(())
    }

    #[async_test]
    async fn test_get_nonexistent_customer() {
        let customer_service = crate_customer_service();

        let customer_id = CustomerId("nonexistent_id".to_string());

        let result = customer_service.get_by_id(&customer_id).await;

        assert!(result.is_none());
    }

    #[async_test]
    async fn test_create_existing_customer() {
        let id_service = Arc::new(IdService::new(UuidRepository));

        let mut mock_repository = MockCustomerRepository::new();
        mock_repository
            .expect_create()
            .times(1)
            .returning(|customer| Err(CustomerError::CustomerAlreadyExist(customer.id().clone())));

        let customer_service = CustomerService::new(id_service, mock_repository);

        let new_customer_command = NewCustomerCommand::new("John Doe".to_string(), false);

        let result = customer_service.create(new_customer_command).await;

        assert!(matches!(result, Err(CustomerError::CustomerAlreadyExist(_))));
    }

    fn crate_customer_service() -> CustomerService {
        let id_service = Arc::new(IdService::new(UuidRepository));
        let customer_repository = CustomerInMemoryRepository::new();
        CustomerService::new(id_service, customer_repository)
    }

}