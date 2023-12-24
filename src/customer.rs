use crate::id::IdService;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct CustomerId(pub String);

impl CustomerId {
    pub fn new(value: &str) -> Self {
        CustomerId(value.to_string())
    }

    pub fn raw(&self) -> String {
        self.0.to_string()
    }
}

impl From<&str> for CustomerId {
    fn from(value: &str) -> Self {
        CustomerId::new(value)
    }
}


#[derive(Clone, Debug)]
pub struct Customer {
    id: CustomerId,
    name: String,
    locked: bool,
}

impl Customer {
    pub fn id(&self) -> &CustomerId {
        &self.id
    }

    pub fn name(&self) -> &String {
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
    id_service: IdService,
    repository: Box<dyn CustomerRepository + Send + Sync>,
}

impl CustomerService {

    pub(crate) fn new(id_service: IdService, repository: impl CustomerRepository + Send + Sync + 'static) -> CustomerService {
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
pub(crate) enum CustomerError {
    CustomerAlreadyExist,
}

pub trait CustomerRepository {
    fn create(&self, customer: Customer) -> Result<Customer, CustomerError>;
    fn get_by_id(&self, customer_id: &CustomerId) -> Option<Customer>;

}