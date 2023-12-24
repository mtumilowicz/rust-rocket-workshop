use rocket::http::Status;
use rocket::{get, post};
use rocket::response::status::{Created, BadRequest, Unauthorized, Custom};
use rocket::serde::json::Json;
use serde_derive::{Deserialize, Serialize};
use uuid::Error;
use crate::customer::{Customer, CustomerError, CustomerId, CustomerService, NewCustomerCommand};

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

impl From<Customer> for CustomerApiOutput {
    fn from(customer: Customer) -> Self {
        CustomerApiOutput {
            id: customer.id().raw(),
            name: customer.name().to_string(),
            locked: customer.locked(),
        }
    }
}

#[post("/customers", data = "<request>")]
pub async fn create_customer(
    request: Json<NewCustomerApiInput>,
    customer_service: &rocket::State<CustomerService>,
) -> Result<Created<Json<CustomerApiOutput>>, Custom<&'static str>> {
    let new_customer: NewCustomerCommand = request.into_inner().into();
    match customer_service.create(new_customer).await {
        Ok(customer) =>  {
            let output: CustomerApiOutput = customer.into();
            Ok(Created::new(output.id.to_string()).body(Json(output)))
        },
        Err(customer_error) => Err(map_customer_error(customer_error)),
    }
}

#[get("/customers/<customer_id>")]
pub async fn get_customer(
    customer_id: &str,
    service: &rocket::State<CustomerService>,
) -> Option<Json<CustomerApiOutput>> {
    let customer_id = &customer_id.into();
    service.get_by_id(customer_id).await
        .map(|r| Json(r.into()))
}

fn map_customer_error(error: CustomerError) -> Custom<&'static str> {
    match error {
        CustomerError::CustomerAlreadyExist =>
            Custom(Status::BadRequest, "customer already exists"),
    }
}