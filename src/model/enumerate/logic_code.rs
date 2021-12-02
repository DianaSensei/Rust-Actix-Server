#[derive(Serialize)]
pub enum StatusCode {
    SUCCESS = 0,
}

impl Into<u16> for StatusCode {
    fn into(self) -> u16 {
        self as u16
    }
}
