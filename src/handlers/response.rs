use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub message: String,
    pub status: String,
}
