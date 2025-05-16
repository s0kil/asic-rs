use crate::data::device::models::MinerModelFactory;
use crate::data::device::{MinerFirmware, MinerMake, MinerModel};
use crate::miners::api::rpc::{btminer::BTMinerV3RPC, traits::SendRPCCommand};
use crate::miners::util;
use diqwest::WithDigestAuth;
use reqwest::{Client, Response};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::net::IpAddr;

pub(crate) async fn get_model_antminer(ip: IpAddr) -> Option<MinerModel> {
    let response: Option<Response> = Client::new()
        .get(format!("http://{}/cgi-bin/get_system_info.cgi", ip))
        .send_with_digest_auth("root", "root")
        .await
        .ok();
    match response {
        Some(data) => {
            let json_data = data.json::<serde_json::Value>().await.ok()?;
            let model = json_data["minertype"].as_str().unwrap_or("").to_uppercase();

            MinerModelFactory::new()
                .with_make(MinerMake::AntMiner)
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

pub(crate) async fn get_model_whatsminer(ip: IpAddr) -> Option<MinerModel> {
    let rpc = BTMinerV3RPC::new(ip, None);
    let device_miner_info = rpc
        .send_command::<GetDeviceMinerInfo>("get.device.info", Some(Box::new("miner")))
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

pub(crate) async fn get_model_luxos(ip: IpAddr) -> Option<MinerModel> {
    let response = util::send_rpc_command(&ip, "version").await;
    match response {
        Some(json_data) => {
            let model = json_data["VERSION"][0]["Type"].as_str();
            if model.is_none() {
                return None;
            }
            let model = model.unwrap().to_uppercase();

            MinerModelFactory::new()
                .with_firmware(MinerFirmware::LuxOS)
                .parse_model(&model)
        }
        None => None,
    }
}

pub(crate) async fn get_model_braiins_os(ip: IpAddr) -> Option<MinerModel> {
    let response = util::send_rpc_command(&ip, "devdetails").await;
    match response {
        Some(json_data) => {
            let model = json_data["DEVDETAILS"][0]["Model"].as_str();
            if model.is_none() {
                return None;
            }
            let model = model
                .unwrap()
                .to_uppercase()
                .replace("BITMAIN ", "")
                .replace("S19XP", "S19 XP");

            MinerModelFactory::new()
                .with_firmware(MinerFirmware::BraiinsOS)
                .parse_model(&model)
        }
        None => None,
    }
}
