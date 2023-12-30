use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket::serde::json::serde_json;
use rust_rocket_workshop::gateway::customer::*;
use rust_rocket_workshop::app;

#[test]
fn test_create_and_get_customer() {
    let rocket = app::server();
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let new_customer_input = NewCustomerApiInput::new(
        "John Doe".to_string(),
    );

    let create_response = client.post("/api/customers")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&new_customer_input).unwrap())
        .dispatch();

    assert_eq!(create_response.status(), Status::Created);

    let customer_id = create_response.headers().get_one("Location").unwrap();

    let get_response = client.get(format!("/api/customers/{}", customer_id))
        .dispatch();

    assert_eq!(get_response.status(), Status::Ok);

    let customer_output: CustomerApiOutput = serde_json::from_str(&get_response.into_string().unwrap()).unwrap();

    assert_eq!(customer_output.id(), customer_id);
    assert_eq!(customer_output.name(), "John Doe");
    assert_eq!(customer_output.locked(), false);
}

#[test]
fn test_get_nonexistent_customer() {
    let rocket = app::server();
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let get_response = client.get("/api/customers/nonexistent_id")
        .dispatch();

    assert_eq!(get_response.status(), Status::NotFound);
}
