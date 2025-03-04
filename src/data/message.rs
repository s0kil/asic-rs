pub enum MessageSeverity {
    Error,
    Warning,
    Info,
}

pub struct MinerMessage {
    timestamp: u32,
    code: i64,
    message: String,
    severity: MessageSeverity,
}
