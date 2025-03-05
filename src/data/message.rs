pub enum MessageSeverity {
    Error,
    Warning,
    Info,
}

pub struct MinerMessage {
    pub timestamp: u32,
    pub code: u64,
    pub message: String,
    pub severity: MessageSeverity,
}
