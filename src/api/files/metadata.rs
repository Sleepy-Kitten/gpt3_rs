use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// The file id used to identify the file
    pub id: String,
    /// The object of the request
    pub object: String,
    /// The size of the file in bytes
    pub bytes: u64,
    /// The time the file was uploaded
    pub created_at: u64,
    /// The name of the file
    pub filename: String,
    /// The purpose of the file
    pub purpose: String,
}
