use rocket::http::Status;
use rocket::{get, post};
use rocket::response::status::{Created, BadRequest};
use rocket::serde::json::Json;
use serde_derive::{Deserialize, Serialize};
use crate::customer::{Customer, CustomerError, CustomerId, CustomerService, NewCustomerCommand};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewCustomerApiInput {
    name: String,
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
        CustomerApiOutput {
            id: customer.id().0.to_string(),
            name: customer.name().to_string(),
            locked: customer.locked(),
        }
    }
}

#[post("/customers", data = "<request>")]
pub async fn create_customer(
    request: Json<NewCustomerApiInput>,
    customer_service: &rocket::State<CustomerService>,
) -> Result<Created<Json<CustomerApiOutput>>, BadRequest<String>> {
    let input = request.into_inner();
    let new_customer = NewCustomerCommand::new(input.name, false);
    match customer_service.create(new_customer).await {
        Ok(customer) =>  {
            let output = CustomerApiOutput::from(customer);
            Ok(Created::new(output.id.to_string()).body(Json(output)))
        },
        Err(customer_error) => {
            match customer_error {
                CustomerError::CustomerAlreadyExist => {
                    Err(BadRequest("Bad request: Customer already exists".to_string()))
                }
            }
        },
    }
}

#[get("/customers/<customer_id>")]
pub async fn get_customer(
    customer_id: &str,
    service: &rocket::State<CustomerService>,
) -> Option<Json<CustomerApiOutput>> {
    service.get_by_id(&CustomerId::new(customer_id)).await
        .map(|r| Json(r.into()))
}