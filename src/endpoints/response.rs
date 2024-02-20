use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ApiResponse {
    pub message: String,
    pub status: String,
}
