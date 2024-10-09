use std::error::Error;

use crate::domain::model::threshold::threshold::Threshold;

pub trait SastParser {
    fn parse(&self, output: &str) -> Result<Vec<ParsedResult>, Box<dyn Error>>;
}

pub struct ParsedResult {
    pub threshold: Threshold,
    pub category: String,
    pub code: String,
    pub file: String,
    pub line: u32
}