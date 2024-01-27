use std::sync::Arc;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket::{Build, Rocket};
use rocket::serde::json::{Value};
use rocket::serde::json::serde_json::json;
use uuid::Uuid;
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
    let expected_result: Value = json!(
        {
            "id": customer_id,
            "name": "John Doe",
            "locked": false
        }
    );

    assert_eq!(get_response.into_json::<Value>(), Some(expected_result));
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
    let expected_result: Value = json!(
        {
            "name": [
                "cannot be empty"
            ]
        }
    );
    assert_eq!(create_response.into_json::<Value>(), Some(expected_result));
}

#[test]
fn test_get_nonexistent_customer() {
    // given
    let rocket = create_server();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let not_existing_id = Uuid::new_v4().to_string();

    // when
    let get_response = client.get(format!("/api/customers/{not_existing_id}"))
        .dispatch();

    // then
    assert_eq!(get_response.status(), Status::NotFound);
}

#[test]
fn test_get_customer_non_uuid() {
    // given
    let rocket = create_server();
    let client = Client::tracked(rocket).expect("valid Rocket instance");

    // when
    let response = client.get("/api/customers/non-uuid").dispatch();

    // then
    assert_eq!(response.status(), Status::UnprocessableEntity);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let expected_result = json!(
        {
            "customer_id": [
                "is not a correct uuid"
            ]
        }
    );
    assert_eq!(response.into_json::<Value>(), Some(expected_result));
}

fn create_server() -> Rocket<Build> {
    let id_service = Arc::new(IdService::new(UuidRepository));
    let customer_service = Arc::new(CustomerService::new(id_service, CustomerInMemoryRepository::new()));
    app::server(
        customer_service
    )
}
