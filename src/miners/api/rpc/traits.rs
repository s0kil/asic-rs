use crate::miners::api::rpc::errors::RPCError;
use serde::de::DeserializeOwned;

pub trait SendRPCCommand {
    async fn send_command<T>(&self, command: &'static str) -> Result<T, RPCError>
    where
        T: DeserializeOwned;

    fn parse_rpc_result<T>(&self, response: &str) -> Result<T, RPCError>
    where
        T: DeserializeOwned;
}
