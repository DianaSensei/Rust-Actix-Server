#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LanguageRequest {
    #[serde(default = "default_language", rename = "language")]
    pub value: String,
}

fn default_language() -> String {
    String::from("vn")
}
