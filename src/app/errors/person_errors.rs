// For person Errors

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PersonError {
    #[error("members not found")]
    PersonNotFound,
    #[error("failed to update members")]
    PersonUpdatefailure,
    #[error("failed to create member")]
    PersonCreateFailure,
    #[error("failed to delete member")]
    PersonDeleteFailure,
}

pub type ErrorMessage = String;

pub trait ResponseErrorTrait {
    fn create(person_error: PersonError) -> ErrorMessage;
}

impl ResponseErrorTrait for ErrorMessage {
    fn create(person_error: PersonError) -> ErrorMessage {
        match person_error {
            PersonError::PersonNotFound => String::from("member not found"),
            PersonError::PersonUpdatefailure => String::from("failed to update member"),
            PersonError::PersonCreateFailure => String::from("failed to create member"),
            PersonError::PersonDeleteFailure => String::from("failed to delete member"),
        }
    }
}