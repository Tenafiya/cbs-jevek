use thiserror::Error;

#[derive(Debug, Error)]
pub enum RustFsError {
    #[error("api error ({status}): {description}")]
    RustfsReqError {
        status: bool,
        description: String,
    }
}