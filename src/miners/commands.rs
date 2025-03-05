#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum MinerCommand {
    RPC { command: &'static str },
    GRPC { command: &'static str },
    WebAPI { command: &'static str, https: bool },
    GraphQL { command: &'static str },
    SSH { command: &'static str },
}

pub(crate) const RPC_DEVDETAILS: MinerCommand = MinerCommand::RPC {
    command: "devdetails",
};
pub(crate) const RPC_VERSION: MinerCommand = MinerCommand::RPC { command: "version" };
pub(crate) const HTTP_WEB_ROOT: MinerCommand = MinerCommand::WebAPI {
    command: "/",
    https: false,
};
pub(crate) const HTTPS_WEB_ROOT: MinerCommand = MinerCommand::WebAPI {
    command: "/",
    https: true,
};
