use std::sync::Arc;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket::{Build, Rocket};
use rocket::serde::json::{serde_json, Value};
use rocket::serde::json::serde_json::json;
use rust_rocket_workshop::app;
use rust_rocket_workshop::domain::customer::CustomerService;
use rust_rocket_workshop::domain::id::IdService;
use rust_rocket_workshop::infrastructure::customer_config::CustomerInMemoryRepository;
use rust_rocket_workshop::infrastructure::id_config::UuidRepository;

#[test]
fn test_create_and_get_customer() {
    // given
    let rocket = create_server();
    let client = Client::tracked(rocket).expect("valid rocket instance");

    // and
    let new_customer_input = r#"
        {
            "name": "John Doe"
        }
    "#;

    // when
    let create_response = client.post("/api/customers")
        .header(ContentType::JSON)
        .body(new_customer_input)
        .dispatch();

    // then
    assert_eq!(create_response.status(), Status::Created);

    // when
    let customer_id = create_response.headers().get_one("Location").unwrap();
    let get_response = client.get(format!("/api/customers/{customer_id}"))
        .dispatch();

    // then
    assert_eq!(get_response.status(), Status::Ok);

    // and
    let result: Value = serde_json::from_str(&get_response.into_string().unwrap()).unwrap();
    let expected_result: Value = json!(
        {
            "id": customer_id,
            "name": "John Doe",
            "locked": false
        }
    );

    assert_eq!(&result, &expected_result);
}

#[test]
fn test_create_validations() {
    // given
    let rocket = create_server();
    let client = Client::tracked(rocket).expect("valid rocket instance");

    // and
    let new_customer_input = r#"
        {
            "name": ""
        }
    "#;

    // when
    let create_response = client.post("/api/customers")
        .header(ContentType::JSON)
        .body(new_customer_input)
        .dispatch();

    // then
    assert_eq!(create_response.status(), Status::UnprocessableEntity);
    let result: Value = serde_json::from_str(&create_response.into_string().unwrap()).unwrap();
    let expected_result: Value = json!(
        {
            "data": {
                "name": [
                    "cannot be empty"
                ]
            }
        }
    );
    assert_eq!(&result, &expected_result);
}

#[test]
fn test_get_nonexistent_customer() {
    // given
    let rocket = create_server();
    let client = Client::tracked(rocket).expect("valid rocket instance");

    // when
    let get_response = client.get("/api/customers/nonexistent_id")
        .dispatch();

    // then
    assert_eq!(get_response.status(), Status::NotFound);
}

fn create_server() -> Rocket<Build> {
    let id_service = Arc::new(IdService::new(UuidRepository));
    let customer_service = Arc::new(CustomerService::new(id_service, CustomerInMemoryRepository::new()));
    app::server(
        customer_service
    )
}
