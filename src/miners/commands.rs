#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MinerCommand {
    RPC { command: &'static str },
    GRPC { command: &'static str },
    WebAPI { command: &'static str, https: bool },
    GraphQL { command: &'static str },
    SSH { command: &'static str },
}
