use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LogEntry {
    #[serde(rename = "type")]
    pub entry_type: Option<String>,
    pub message: Option<MessageEntry>,
}

#[derive(Debug, Deserialize)]
pub struct MessageEntry {
    pub usage: Option<UsageEntry>,
}

#[derive(Debug, Deserialize)]
pub struct UsageEntry {
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
}
