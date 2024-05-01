use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref COMMIT_HASH_REGEX: Regex = Regex::new(r"^[0-9a-fA-F]{40}$").unwrap();
}

pub fn commit_hash_valid(hash: &str) -> bool {
    COMMIT_HASH_REGEX.is_match(hash)
}
