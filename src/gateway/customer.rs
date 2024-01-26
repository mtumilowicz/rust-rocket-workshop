use std::sync::Arc;
use rocket::http::Status;
use rocket::{get, post};
use rocket::response::status::{Created, Custom};
use rocket::serde::json::Json;
use serde_derive::{Deserialize, Serialize};
use crate::domain::customer::{Customer, CustomerError, CustomerService, NewCustomerCommand};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewCustomerApiInput {
    name: String,
}

impl Into<NewCustomerCommand> for NewCustomerApiInput {
    fn into(self) -> NewCustomerCommand {
        NewCustomerCommand::new(self.name, false)
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CustomerApiOutput {
    id: String,
    name: String,
    locked: bool,
}

impl CustomerApiOutput {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn locked(&self) -> bool {
        self.locked
    }
}

impl From<Customer> for CustomerApiOutput {
    fn from(customer: Customer) -> Self {
        CustomerApiOutput {
            id: customer.id().raw(),
            name: customer.name().to_string(),
            locked: customer.locked(),
        }
    }
}

impl From<CustomerError> for Custom<String> {
    fn from(value: CustomerError) -> Self {
        match value {
            CustomerError::CustomerAlreadyExist(customer_id) =>
                Custom(Status::BadRequest, format!("customer with id = {customer_id} already exists"))
        }
    }
}

#[post("/customers", data = "<request>")]
pub async fn create_customer(
    request: Json<NewCustomerApiInput>,
    customer_service: &rocket::State<Arc<CustomerService>>,
) -> Result<Created<Json<CustomerApiOutput>>, Custom<String>> {
    let new_customer: NewCustomerCommand = request.into_inner().into();
    customer_service.create(new_customer).await
        .map(|customer| {
            let output: CustomerApiOutput = customer.into();
            Created::new(output.id.to_string()).body(Json(output))
        })
        .map(|result| Ok(result))?
}

#[get("/customers/<customer_id>")]
pub async fn get_customer(
    customer_id: String,
    service: &rocket::State<Arc<CustomerService>>,
) -> Option<Json<CustomerApiOutput>> {
    let customer_id = &customer_id.into();
    service.get_by_id(customer_id).await
        .map(|r| Json(r.into()))
}