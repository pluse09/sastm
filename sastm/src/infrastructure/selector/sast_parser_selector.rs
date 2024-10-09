// DomainServiceあたりに書きたかったがinfra層の実装クラスを扱うので一旦ここに。

use crate::domain::model::sast::sast_tool_name::SastToolName;
use crate::domain::model::parser::sast_parser::SastParser;
use crate::infrastructure::parser::brakeman_parser::BrakemanParser;

pub struct SastParserSelector;

impl SastParserSelector {

    pub fn execute(sast_tool_name: SastToolName) -> Option<Box<dyn SastParser>> {
        let tmp = sast_tool_name.value().to_lowercase();
        match tmp.as_str() {
            "brakeman" => Some(Box::new(BrakemanParser::new())),
            // "semgrep" => Box::new(SemgrepParser::new()),
            _ => None,
        }
    }

}