use std::env;
use std::string::ToString;
use once_cell::sync::Lazy;
use std::str::FromStr;

#[derive(EnumString, Display, PartialEq)]
pub enum Profile {
    #[strum(ascii_case_insensitive)]
    DEVELOPMENT,

    #[strum(ascii_case_insensitive)]
    TESTING,

    #[strum(ascii_case_insensitive)]
    STAGING,

    #[strum(ascii_case_insensitive)]
    PRODUCTION
}

pub static PROFILE: Lazy<Profile> = Lazy::new(get_profile);

pub fn get_profile() -> Profile {
    let profile = env::var("PROFILE").unwrap_or(Profile::DEVELOPMENT.to_string());
    Profile::from_str(profile.as_str()).unwrap_or(Profile::DEVELOPMENT)
}