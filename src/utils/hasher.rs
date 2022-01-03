use crate::settings;
use libreauth::pass::{Algorithm, HashBuilder, Hasher};
use once_cell::sync::Lazy;

const PWD_ALGORITHM: Algorithm = Algorithm::Argon2;
static PWD_SCHEME_VERSION: Lazy<usize> = Lazy::new(|| settings::SETTINGS.hasher.scheme_version);

// If the Hasher changes, make sure to increment PWD_SCHEME_VERSION
static HASHER: Lazy<Hasher> = Lazy::new(|| {
    info!(
        "HASHER ARGON 2 INITIATE: - version {} [SUCCESS]",
        *PWD_SCHEME_VERSION
    );

    HashBuilder::new()
        .algorithm(PWD_ALGORITHM)
        .version(*PWD_SCHEME_VERSION)
        .finalize()
        .unwrap()
});

pub fn default_hasher_scheme_version() -> usize {
    1
}

pub fn get_argon2_hasher() -> &'static Hasher {
    &HASHER
}

pub fn hash_validation(store: String, need: String) -> bool {
    let checker = HashBuilder::from_phc(store.as_str()).unwrap();
    checker.is_valid(need.as_str())
}
