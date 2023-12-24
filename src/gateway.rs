use rocket::http::Status;
use rocket::post;
use rocket::response::status::{Created, BadRequest};
use rocket::serde::json::Json;
use serde_derive::{Deserialize, Serialize};
use crate::customer::{Customer, CustomerError, CustomerService, NewCustomerCommand};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewCustomerApiInput {
    name: String,
}

impl From<NewCustomerApiInput> for NewCustomerCommand {
    fn from(api_input: NewCustomerApiInput) -> Self {
        NewCustomerCommand::new(api_input.name, false)
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CustomerApiOutput {
    id: String,
    name: String,
    locked: bool,
}

impl From<Customer> for CustomerApiOutput {
    fn from(customer: Customer) -> Self {
        customer.into()
    }
}

#[post("/customers", data = "<request>")]
pub async fn create_customer(
    request: Json<NewCustomerApiInput>,
    customer_service: &rocket::State<CustomerService>,
) -> Result<Created<Json<CustomerApiOutput>>, BadRequest<String>> {
    match customer_service.create(request.into_inner().into()).await {
        Ok(output) => Ok(Created::new("/resource.json").body(Json(output.into()))),
        Err(customer_error) => {
            match customer_error {
                CustomerError::CustomerAlreadyExist => {
                    Err(BadRequest("Bad request: Customer already exists".to_string()))
                }
            }
        },
    }
}