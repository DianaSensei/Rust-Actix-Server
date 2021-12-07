#[allow(clippy::upper_case_acronyms)]
#[derive(Serialize)]
pub enum StatusCode {
    SUCCESS = 0,
}

impl From<StatusCode> for u16 {
    fn from(status: StatusCode) -> Self {
        status as u16
    }
}
