use rocket::async_test;
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;
use rocket::serde::json::serde_json;
use rust_rocket_workshop::gateway::customer::*;
use rust_rocket_workshop::app;

#[async_test]
async fn test_create_and_get_customer() {
    let rocket = app::server().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let new_customer_input = r#"
        {
            "name": "John Doe"
        }
    "#;

    let create_response = client.post("/api/customers")
        .header(ContentType::JSON)
        .body(new_customer_input)
        .dispatch()
        .await;

    assert_eq!(create_response.status(), Status::Created);

    let customer_id = create_response.headers().get_one("Location").unwrap();

    let get_response = client.get(format!("/api/customers/{}", customer_id))
        .dispatch()
        .await;

    assert_eq!(get_response.status(), Status::Ok);

    let customer_output: CustomerApiOutput = serde_json::from_str(&get_response.into_string().await.unwrap()).unwrap();

    assert_eq!(customer_output.id(), customer_id);
    assert_eq!(customer_output.name(), "John Doe");
    assert_eq!(customer_output.locked(), false);
}

#[async_test]
async fn test_get_nonexistent_customer() {
    let rocket = app::server().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let get_response = client.get("/api/customers/nonexistent_id")
        .dispatch()
        .await;

    assert_eq!(get_response.status(), Status::NotFound);
}
