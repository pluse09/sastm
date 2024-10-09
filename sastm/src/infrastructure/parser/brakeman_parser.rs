use serde::Deserialize;
use std::error::Error;

use crate::domain::model::{parser::sast_parser::{ParsedResult, SastParser}, threshold::threshold::Threshold};

pub struct BrakemanParser;

impl BrakemanParser {
    pub fn new() -> Self {
        BrakemanParser
    }
}

impl SastParser for BrakemanParser {

    fn parse(&self, output: &str) -> Result<Vec<ParsedResult>, Box<dyn Error>> {
        let mut parsed_results: Vec<ParsedResult> = Vec::new();

        let brakeman_result: Result<BrakemanOutputDTO, serde_json::Error> = serde_json::from_str(output);

        match brakeman_result {
            Ok(brakeman_result) => {
                for warning in brakeman_result.warnings {
                    let threshold = Threshold::from_str(warning.confidence.as_deref().unwrap_or("Unknown"));
                    let parsed_result = ParsedResult {
                        threshold,
                        category: warning.warning_type.unwrap_or_else(|| "Unknown".to_string()),
                        code: warning.code.unwrap_or_else(|| "N/A".to_string()),
                        file: warning.file.unwrap_or_else(|| "Unknown".to_string()),
                        line: warning.line,
                    };
                    parsed_results.push(parsed_result);
                }
            }
            Err(e) => {
                // eprintln!("Failed to parse Brakeman output: {}", e);
                Err(e)
            }
        }

    }
}



#[derive(Deserialize, Debug)]
pub struct Location {
    r#type: Option<String>,
    class: Option<String>,
    method: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Rendered {
    name: Option<String>,
    file: Option<String>,
}

#[derive(Deserialize, Debug)]
struct RenderPath {
    r#type: Option<String>,
    class: Option<String>,
    method: Option<String>,
    line: Option<u32>,
    file: Option<String>,
    rendered: Option<Rendered>,
}

#[derive(Deserialize, Debug)]
pub struct Warning {
    warning_type: Option<String>,
    warning_code: u32,
    fingerprint: Option<String>,
    check_name: Option<String>,
    message: Option<String>,
    file: Option<String>,
    line: u32,
    link: Option<String>,
    code: Option<String>,
    render_path: Option<Vec<RenderPath>>,
    location: Option<Location>,
    user_input: Option<String>,
    confidence: Option<String>,
    cwe_id: Option<Vec<u32>>,
}

#[derive(Deserialize, Debug)]
pub struct BrakemanOutputDTO {
    pub warnings: Vec<Warning>,
    pub ignored_warnings: Vec<Warning>,
    pub errors: Vec<String>,
}

// pub fn parse_brakeman_output(json_content: &str) -> Result<BrakemanOutputDTO, SerdeError> {
//     serde_json::from_str(json_content)
//     // TODO: こうぞくのコードを追加
// }