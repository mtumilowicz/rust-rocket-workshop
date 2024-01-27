use std::borrow::Cow;
use std::collections::HashMap;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use serde_derive::Serialize;
use validator::{ValidationErrors};


#[derive(Serialize)]
pub struct ErrorApiOutput(HashMap<&'static str, HashMap<&'static str, Vec<Cow<'static, str>>>>);

impl ErrorApiOutput {

    pub fn from(key: &'static str, errors: ValidationErrors) -> ErrorApiOutput {
        let mut error_map = HashMap::new();

        for (field, validation_errors) in errors.field_errors() {
            let error_messages: Vec<_> = validation_errors
                .iter()
                .flat_map(|error| error.message.clone())
                .collect();

            error_map.insert(field, error_messages);
        }

        ErrorApiOutput(HashMap::from([(key, error_map)]))
    }

    pub fn for_key(key: &'static str, message: Cow<'static, str>) -> ErrorApiOutput {
        let mut data = HashMap::new();
        data.insert("error", vec!(message));

        ErrorApiOutput(HashMap::from([(key, data)]))
    }
}

impl From<ErrorApiOutput> for Custom<Json<ErrorApiOutput>> {
    fn from(value: ErrorApiOutput) -> Self {
        Custom(Status::UnprocessableEntity, Json(value))
    }
}