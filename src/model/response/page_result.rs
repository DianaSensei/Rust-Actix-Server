use std::fmt;
use serde::Serialize;
use crate::model::domain::pagination_result::PaginationResult;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageResponse<T> {
    pub data: Vec<T>,
    pub pageable: PaginationResult
}

impl <T>fmt::Display for PageResponse<T>
where T: Serialize
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}