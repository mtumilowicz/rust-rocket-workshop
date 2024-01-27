use std::borrow::Cow;
use std::collections::HashMap;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use serde_derive::Serialize;
use validator::{ValidationErrors};



#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum ErrorApiOutput {
    #[serde(rename = "error")]
    Error(Cow<'static, str>),
    #[serde(rename = "errors")]
    Errors(HashMap<&'static str, Cow<'static, str>>)
}
impl ErrorApiOutput {

    pub fn validation_errors(errors: ValidationErrors) -> ErrorApiOutput {
        let mut error_map = HashMap::new();

        for (field, validation_errors) in errors.field_errors() {
            let error_messages: Vec<_> = validation_errors
                .iter()
                .flat_map(|error| error.message.clone())
                .collect();

            if let Some(error_message) = error_messages.first() {
                error_map.insert(field, error_message.to_owned());
            }
        }

        ErrorApiOutput::Errors(error_map)
    }

    pub fn error(message: Cow<'static, str>) -> ErrorApiOutput {
        ErrorApiOutput::Error(message)
    }
}

impl From<ErrorApiOutput> for Custom<Json<ErrorApiOutput>> {
    fn from(value: ErrorApiOutput) -> Self {
        Custom(Status::UnprocessableEntity, Json(value))
    }
}