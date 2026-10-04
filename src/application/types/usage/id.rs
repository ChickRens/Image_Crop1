pub struct UsageId {
    value: String
}

impl UsageId {
    pub fn new(value: &str) -> Self {
        Self { value: value.to_string() }
    }
}

