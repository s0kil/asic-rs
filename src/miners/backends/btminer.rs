use std::net::IpAddr;

use crate::miners::api::rpc::errors::RPCError;
use crate::miners::api::rpc::{btminer::BTMinerV3RPC, traits::SendRPCCommand};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub struct BTMinerV3Backend {
    rpc: BTMinerV3RPC,
}

impl BTMinerV3Backend {
    pub fn new(ip: IpAddr) -> Self {
        BTMinerV3Backend {
            rpc: BTMinerV3RPC::new(ip, None),
        }
    }
    pub async fn get_device_info(&self) -> Result<GetDeviceInfo, RPCError> {
        self.rpc
            .send_command::<GetDeviceInfo>("get.device.info")
            .await
    }
}

#[derive(Debug)]
pub struct GetDeviceInfo {
    pub api_version: Option<String>,
}

impl<'de> Deserialize<'de> for GetDeviceInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Value::deserialize(deserializer)?;

        let api_version = value["msg"]["system"]["api"].as_str();

        Ok(GetDeviceInfo {
            api_version: api_version.map(|s| s.to_string()),
        })
    }
}
