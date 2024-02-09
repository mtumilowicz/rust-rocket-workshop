use std::borrow::Cow;
use std::collections::HashMap;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde_derive::Serialize;
use thiserror::Error;
use validator::{ValidationErrors};

#[derive(Serialize, JsonSchema, Error, Debug)]
#[serde(crate = "rocket::serde")]
pub enum ErrorApiOutput {
    #[serde(rename = "errors")]
    #[error("errors: {0:?}")]
    Unprocessable(HashMap<&'static str, Vec<Cow<'static, str>>>),
    #[serde(rename = "error")]
    #[error("error: {0}")]
    Error(Cow<'static, str>),
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

        ErrorApiOutput::Unprocessable(error_map)
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
        match &value {
            ErrorApiOutput::Unprocessable(_) => Custom(Status::UnprocessableEntity, Json(value)),
            ErrorApiOutput::Error(_) => Custom(Status::BadRequest, Json(value)),
        }
    }
}

#[derive(Serialize, JsonSchema, Error, Debug)]
#[error("error: {0}")]
pub struct CannotProcessEntity(pub String);

impl From<CannotProcessEntity> for Custom<Json<ErrorApiOutput>> {
    fn from(value: CannotProcessEntity) -> Self {
        Custom(Status::UnprocessableEntity, Json(ErrorApiOutput::error_string(value.0)))
    }
}