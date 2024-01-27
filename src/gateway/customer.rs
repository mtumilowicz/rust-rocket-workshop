use std::borrow::Cow;
use std::sync::Arc;
use rocket::http::Status;
use rocket::{get, post};
use rocket::response::status::{Created, Custom};
use rocket::serde::json::Json;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate};
use crate::domain::customer::{Customer, CustomerError, CustomerId, CustomerService, NewCustomerCommand};
use crate::gateway::error::{ErrorApiOutput};

#[derive(Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct NewCustomerApiInput {
    #[validate(length(min = 1, message = "cannot be empty"))]
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
            id: customer.id().to_string(),
            name: customer.name().to_string(),
            locked: customer.locked(),
        }
    }
}

impl From<CustomerError> for Custom<Json<ErrorApiOutput>> {
    fn from(value: CustomerError) -> Self {
        match value {
            CustomerError::CustomerAlreadyExist(customer_id) => {
                let message = format!("Customer with id = {customer_id} already exists");
                let output = ErrorApiOutput::for_key("customer", Cow::Owned(message));
                Custom(Status::BadRequest, Json(output))
            }
        }
    }
}

#[post("/customers", data = "<request>")]
pub async fn create_customer(
    request: Json<NewCustomerApiInput>,
    customer_service: &rocket::State<Arc<CustomerService>>,
) -> Result<Created<Json<CustomerApiOutput>>, Custom<Json<ErrorApiOutput>>> {
    request.validate().map_err(|err| ErrorApiOutput::from(err))?;
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
) -> Result<Option<Json<CustomerApiOutput>>, Custom<Json<ErrorApiOutput>>> {
    let customer_id = parse_customer_id(&customer_id)?;
    Ok(service.get_by_id(&customer_id).await
        .map(|r| Json(r.into())))
}

fn parse_customer_id(customer_id: &str) -> Result<CustomerId, Custom<Json<ErrorApiOutput>>> {
    match Uuid::parse_str(customer_id) {
        Ok(uuid) => Ok(CustomerId::new(uuid)),
        Err(_) => {
            let output = ErrorApiOutput::for_key("customer_id", Cow::Borrowed("is not a correct uuid"));
            Err(Custom(Status::UnprocessableEntity, Json(output)))
        }
    }
}