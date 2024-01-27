use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref ENGLISH_ALPHABET: Regex = Regex::new("^[A-Za-z]+$").expect("Failed to compile regex pattern");
}