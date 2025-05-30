use crate::miners::api::rpc::errors::RPCError;
use async_trait::async_trait;
use serde::Serialize;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait SendRPCCommand {
    async fn send_command<T, P>(
        &self,
        command: &'static str,
        param: Option<P>,
    ) -> Result<T, RPCError>
    where
        T: DeserializeOwned,
        P: Serialize + Send;

    fn parse_rpc_result<T>(&self, response: &str) -> Result<T, RPCError>
    where
        T: DeserializeOwned;
}
