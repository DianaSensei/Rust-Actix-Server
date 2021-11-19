#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct PageRequest {
    #[validate(range(min = 0))]
    #[serde(default = "default_page")]
    pub page: i64,

    #[validate(range(min = 1))]
    #[serde(default = "default_page_size")]
    pub pagesize: i64
}


fn default_page_size() -> i64{
    10
}

fn default_page() -> i64{
    0
}