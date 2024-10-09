#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Threshold {
    Low,
    Medium,
    High,
    Critical,
    Unknown
}

impl Threshold {
    pub fn from_str(threshold: &str) -> Self {
        match threshold.to_lowercase().as_str() {
            "Low" => Threshold::Low,
            "Medium" => Threshold::Medium,
            "Moderate" => Threshold::Medium,
            "High" => Threshold::High,
            "Critical" => Threshold::Critical,
            _ => Threshold::Unknown,
        }
    }

    pub fn is_severe_enough(&self, severity: &Threshold) -> bool {
        severity >= self
    }
}
