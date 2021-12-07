#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}
