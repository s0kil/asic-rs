use crate::miners::api::rpc::errors::RPCError;
use dyn_serde::Serialize;
use serde::de::DeserializeOwned;

pub trait SendRPCCommand {
    fn send_command<T>(
        &self,
        command: &'static str,
        param: Option<Box<dyn Serialize + Send>>,
    ) -> impl std::future::Future<Output = Result<T, RPCError>> + Send
    where
        T: DeserializeOwned;

    fn parse_rpc_result<T>(&self, response: &str) -> Result<T, RPCError>
    where
        T: DeserializeOwned;
}
