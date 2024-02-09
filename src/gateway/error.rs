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

    pub fn unprocessable(key: &'static str, message: Cow<'static, str>) -> Self {
        let mut error_map = HashMap::new();
        error_map.insert(key, vec![message]);
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
#[error("error: {values}")]
pub struct CannotProcessEntity { key: &'static str, values: Cow<'static, str> }
impl CannotProcessEntity {

    pub fn message_str(value: &'static str) -> Self {
        Self::from_str("message", value)
    }

    pub fn message_string(value: String) -> Self {
        Self::from_string("message", value)
    }

    pub fn from_str(key: &'static str, value: &'static str) -> Self {
        CannotProcessEntity {
            key: key,
            values: Cow::Borrowed(value) }
    }

    pub fn from_string(key: &'static str, value: String) -> Self {
        CannotProcessEntity {
            key: key,
            values: Cow::Owned(value)
        }
    }
}

impl From<CannotProcessEntity> for ErrorApiOutput {
    fn from(value: CannotProcessEntity) -> Self {
        ErrorApiOutput::unprocessable(value.key, value.values)
    }
}