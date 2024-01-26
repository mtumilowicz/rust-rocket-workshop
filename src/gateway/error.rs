use std::collections::HashMap;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use serde_derive::Serialize;
use validator::{ValidationErrors};


#[derive(Serialize)]
pub struct ErrorApiOutput {
    data: HashMap<String, Vec<String>>
}

impl ErrorApiOutput {
    pub fn new(data: HashMap<String, Vec<String>>) -> Self {
        ErrorApiOutput { data }
    }

    pub fn from(errors: ValidationErrors) -> ErrorApiOutput {
        let mut error_map = HashMap::new();

        for (field, validation_errors) in errors.field_errors() {
            let error_messages: Vec<String> = validation_errors
                .iter()
                .flat_map(|error| error.message.clone().map(|msg| msg.to_string()))
                .collect();

            error_map.insert(field.to_string(), error_messages);
        }

        ErrorApiOutput::new(error_map)
    }
}

impl From<ErrorApiOutput> for Custom<Json<ErrorApiOutput>> {
    fn from(value: ErrorApiOutput) -> Self {
        Custom(Status::UnprocessableEntity, Json(value))
    }
}