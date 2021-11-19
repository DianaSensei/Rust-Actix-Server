use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaginationResult {
    pub number_of_elements: usize,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
    pub total_elements: usize
}

impl fmt::Display for PaginationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}