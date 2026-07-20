use std::time::Duration;

#[derive(Debug)]
pub struct GenUploadUrlPayload {
    pub file_key: String,
    pub bucket: String,
    pub expires_in: Duration,
}

#[derive(Debug)]
pub struct FileExistPayload {
    pub file_key: String,
    pub bucket: String,
}