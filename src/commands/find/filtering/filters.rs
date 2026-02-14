use regex::Regex;

/// Filter trait that defines matching behavior
pub trait Filter<FilterValue> {
    fn is_matched(&self, filter_value: FilterValue) -> bool;
}

/// Filter that compares values for equality
pub struct ExactMatchFilter<CompareValue: PartialEq> {
    compare_value: CompareValue,
}

impl<CompareValue: PartialEq> ExactMatchFilter<CompareValue> {
    pub fn new(compare_value: CompareValue) -> Self {
        Self { compare_value }
    }
}

impl<Value: PartialEq> Filter<Value> for ExactMatchFilter<Value> {
    fn is_matched(&self, filter_value: Value) -> bool {
        self.compare_value == filter_value
    }
}

/// Filter that check if regex matched
pub struct RegexFilter {
    regex: Regex,
}

impl RegexFilter {
    pub fn new(regex_pattern: &str) -> anyhow::Result<Self> {
        let regex = Regex::new(regex_pattern)?;
        Ok(RegexFilter { regex })
    }
}

impl Filter<String> for RegexFilter {
    fn is_matched(&self, filter_value: String) -> bool {
        self.regex.is_match(&filter_value)
    }
}
