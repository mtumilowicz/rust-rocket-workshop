use rocket::async_test;
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;
use rocket::serde::json::{serde_json, Value};
use rocket::serde::json::serde_json::json;
use rust_rocket_workshop::app;

#[async_test]
async fn test_create_and_get_customer() {
    // given
    let rocket = app::server().await;
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

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
        .dispatch()
        .await;

    // then
    assert_eq!(create_response.status(), Status::Created);

    // when
    let customer_id = create_response.headers().get_one("Location").unwrap();
    let get_response = client.get(format!("/api/customers/{}", customer_id))
        .dispatch()
        .await;

    // then
    assert_eq!(get_response.status(), Status::Ok);

    // and
    let result: Value = serde_json::from_str(&get_response.into_string().await.unwrap()).unwrap();
    let expected_result: Value = json!(
        {
            "id": customer_id,
            "name": "John Doe",
            "locked": false
        }
    );

    assert_eq!(&result, &expected_result);
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
