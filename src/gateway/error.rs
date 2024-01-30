use std::borrow::Cow;
use std::collections::HashMap;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use serde_derive::Serialize;
use thiserror::Error;
use validator::{ValidationErrors};



#[derive(Serialize, Error, Debug)]
#[serde(crate = "rocket::serde")]
pub enum ErrorApiOutput {
    #[serde(rename = "error")]
    #[error("error: {0}")]
    Error(Cow<'static, str>),
    #[serde(rename = "errors")]
    #[error("errors: {0:?}")]
    Errors(HashMap<&'static str, Vec<Cow<'static, str>>>)
}
impl ErrorApiOutput {

    pub fn validation_errors(errors: ValidationErrors) -> ErrorApiOutput {
        let mut error_map = HashMap::new();

        for (field, validation_errors) in errors.field_errors() {
            let error_messages = validation_errors
                .iter()
                .flat_map(|error| error.message.clone())
                .collect();

            error_map.insert(field, error_messages);
        }

        ErrorApiOutput::Errors(error_map)
    }

    pub fn error_str(message: &'static str) -> Self {
        ErrorApiOutput::Error(Cow::Borrowed(message))
    }

    pub fn error_string(message: String) -> Self {
        ErrorApiOutput::Error(Cow::Owned(message))
    }
}

impl From<ErrorApiOutput> for Custom<Json<ErrorApiOutput>> {
    fn from(value: ErrorApiOutput) -> Self {
        Custom(Status::UnprocessableEntity, Json(value))
    }
}