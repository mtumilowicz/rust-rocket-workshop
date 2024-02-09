use std::sync::Arc;
use rocket::http::Status;
use rocket::{get, post};
use rocket::request::FromParam;
use rocket::response::status::{Created, Custom};
use rocket::serde::json::Json;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{openapi};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate};
use crate::domain::customer::{Customer, CustomerError, CustomerId, CustomerService, NewCustomerCommand};
use crate::gateway::error::{CannotProcessEntity, ErrorApiOutput};
use crate::gateway::regex::ENGLISH_ALPHABET;

type ApiIO<T> = Result<T, Custom<Json<ErrorApiOutput>>>;

#[derive(Deserialize, Validate, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct NewCustomerApiInput {
    #[validate(length(min = 1, message = "cannot be empty"))]
    #[validate(regex(path = "ENGLISH_ALPHABET", message = "does not contain only letters"))]
    name: String,
}

impl Into<NewCustomerCommand> for NewCustomerApiInput {
    fn into(self) -> NewCustomerCommand {
        NewCustomerCommand::new(self.name, false)
    }
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct CustomerApiOutput {
    id: String,
    name: String,
    locked: bool,
}

impl<'r> FromParam<'r> for CustomerId {
    type Error = CannotProcessEntity;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        match Uuid::parse_str(param) {
            Ok(uuid) => Ok(CustomerId::new(uuid)),
            Err(_) => Err(CannotProcessEntity::from_str("customer_id", "not a correct uuid"))
        }
    }
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
                let output = ErrorApiOutput::error_string(message);
                Custom(Status::BadRequest, Json(output))
            }
        }
    }
}

#[openapi(tag = "Customers")]
#[post("/customers", data = "<request>")]
pub async fn create_customer(
    request: Json<NewCustomerApiInput>,
    customer_service: &rocket::State<Arc<CustomerService>>,
) -> ApiIO<Created<Json<CustomerApiOutput>>> {
    request.validate().map_err(|err| ErrorApiOutput::validation_errors(err))?;
    let new_customer: NewCustomerCommand = request.into_inner().into();
    customer_service.create(new_customer).await
        .map(|customer| {
            let output: CustomerApiOutput = customer.into();
            Created::new(output.id.to_string()).body(Json(output))
        })
        .map(|result| Ok(result))?
}

#[openapi(tag = "Customers")]
#[get("/customers/<customer_id>")]
pub async fn get_customer(
    customer_id: Result<CustomerId, CannotProcessEntity>,
    service: &rocket::State<Arc<CustomerService>>,
) -> Result<Option<Json<CustomerApiOutput>>, Custom<Json<ErrorApiOutput>>> {
    let customer_id = customer_id.map_err(ErrorApiOutput::from)?;
    Ok(service.get_by_id(&customer_id).await
        .map(|r| Json(r.into())))
}