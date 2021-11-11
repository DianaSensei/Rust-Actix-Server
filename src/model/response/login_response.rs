#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub data: String,
    pub status: i64,
    pub request_id: String,
    pub auth_token: String,
}