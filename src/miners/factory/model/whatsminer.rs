use crate::data::device::models::MinerModelFactory;
use crate::data::device::{MinerMake, MinerModel};
use crate::miners::api::rpc::{btminer::BTMinerV3RPC, traits::SendRPCCommand};
use crate::miners::util;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::net::IpAddr;

pub(crate) async fn get_model_whatsminer_v2(ip: IpAddr) -> Option<MinerModel> {
    let response = util::send_rpc_command(&ip, "devdetails").await;
    match response {
        Some(json_data) => {
            let model = json_data["DEVDETAILS"][0]["Model"].as_str();
            if model.is_none() {
                return None;
            }
            let mut model = model.unwrap().to_uppercase().replace("_", "");
            model.pop();
            model.push('0');

            MinerModelFactory::new()
                .with_make(MinerMake::WhatsMiner)
                .parse_model(&model)
        }
        None => None,
    }
}

#[derive(Debug)]
pub struct GetDeviceMinerInfo {
    pub model: Option<String>,
}

impl<'de> Deserialize<'de> for GetDeviceMinerInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Value::deserialize(deserializer)?;

        let model = value["msg"]["miner"]["type"]
            .as_str()
            .map(|s| s.to_string());

        Ok(Self { model })
    }
}

pub(crate) async fn get_model_whatsminer_v3(ip: IpAddr) -> Option<MinerModel> {
    let rpc = BTMinerV3RPC::new(ip, None);
    let device_miner_info = rpc
        .send_command::<GetDeviceMinerInfo, &str>("get.device.info", Some("miner"))
        .await;

    match device_miner_info {
        Ok(data) => {
            if data.model.is_none() {
                return None;
            }
            let mut model = data.model.unwrap().to_uppercase().replace("_", "");
            model.pop();
            model.push('0');

            MinerModelFactory::new()
                .with_make(MinerMake::WhatsMiner)
                .parse_model(&model)
        }
        Err(_) => None,
    }
}
