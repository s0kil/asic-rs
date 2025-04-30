use std::net::IpAddr;
use std::str::FromStr;

use crate::data::fan::FanData;
use crate::miners::api::rpc::errors::RPCError;
use crate::miners::api::rpc::{btminer::BTMinerV3RPC, traits::SendRPCCommand};
use macaddr::MacAddr;
use measurements::{AngularVelocity, Power, Voltage};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub struct BTMinerV3Backend {
    pub rpc: BTMinerV3RPC,
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
    pub fw_version: Option<String>,
    pub control_board_version: Option<String>,
    pub mac: Option<MacAddr>,
    pub serial_number: Option<String>,
    pub hostname: Option<String>,
    pub psu_fans: Vec<FanData>,
    pub light_flashing: Option<bool>,
    pub wattage_limit: Option<Power>,
    pub voltage: Option<Voltage>,
    pub board_sns: Vec<String>,
}

impl<'de> Deserialize<'de> for GetDeviceInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Value::deserialize(deserializer)?;

        let api_version = value["msg"]["system"]["api"]
            .as_str()
            .map(|s| s.to_string());

        let fw_version = value["msg"]["system"]["fwversion"]
            .as_str()
            .map(|s| s.to_string());

        let control_board_version = value["msg"]["system"]["platform"]
            .as_str()
            .map(|s| s.to_string());

        let mac = value["msg"]["network"]["mac"]
            .as_str()
            .and_then(|s| MacAddr::from_str(s).ok());

        let serial_number = value["msg"]["miner"]["miner-sn"]
            .as_str()
            .map(|s| s.to_string());

        let hostname = value["msg"]["network"]["hostname"]
            .as_str()
            .map(|s| s.to_string());

        let light_flashing = value["msg"]["system"]["ledstatus"]
            .as_str()
            .map(|s| s != "auto");

        let wattage_limit = value["msg"]["miner"]["power-limit-set"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .map(|f| Power::from_watts(f));

        let voltage = value["msg"]["power"]["vout"]
            .as_f64()
            .map(|f| Voltage::from_millivolts(f));

        let mut psu_fans: Vec<FanData> = Vec::new();

        value["msg"]["power"]["fanspeed"].as_f64().map(|f| {
            psu_fans.push(FanData {
                position: 0,
                rpm: AngularVelocity::from_rpm(f),
            })
        });

        let mut board_sns: Vec<String> = Vec::new();

        for idx in 0..3 {
            let board_sn = value["msg"]["miner"][format!("pcbsn{}", idx)].as_str();
            if board_sn.is_some() {
                board_sns.push(board_sn.unwrap().to_owned());
            }
        }

        Ok(GetDeviceInfo {
            api_version,
            fw_version,
            control_board_version,
            mac,
            serial_number,
            hostname,
            psu_fans,
            light_flashing,
            wattage_limit,
            voltage,
            board_sns,
        })
    }
}
