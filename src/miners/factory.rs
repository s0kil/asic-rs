use std::net::IpAddr;
use std::{collections::HashSet, error::Error};

use reqwest::StatusCode;
use reqwest::header::HeaderMap;

use super::commands::{HTTP_WEB_ROOT, MinerCommand, RPC_VERSION};
use crate::data::device::{DeviceInfo, MinerFirmware, MinerMake};

use super::util::{send_rpc_command, send_web_command};

pub(crate) trait DiscoveryCommands {
    fn into_discovery_commands(&self) -> Vec<MinerCommand>;
}

impl DiscoveryCommands for MinerMake {
    fn into_discovery_commands(&self) -> Vec<MinerCommand> {
        match self {
            MinerMake::AntMiner => vec![RPC_VERSION, HTTP_WEB_ROOT],
            MinerMake::WhatsMiner => vec![],
            MinerMake::AvalonMiner => vec![],
            MinerMake::EPic => vec![],
            MinerMake::Braiins => vec![],
            MinerMake::BitAxe => vec![],
        }
    }
}
impl DiscoveryCommands for MinerFirmware {
    fn into_discovery_commands(&self) -> Vec<MinerCommand> {
        match self {
            MinerFirmware::Stock => vec![], // stock firmware needs miner make
            MinerFirmware::BraiinsOS => vec![RPC_VERSION, HTTP_WEB_ROOT],
            MinerFirmware::VNish => vec![],
            MinerFirmware::EPic => vec![],
            MinerFirmware::HiveOn => vec![],
            MinerFirmware::LuxOS => vec![],
            MinerFirmware::Marathon => vec![],
            MinerFirmware::MSKMiner => vec![],
        }
    }
}

pub async fn get_miner(
    ip: &IpAddr,
    makes: Option<Vec<MinerMake>>,
    firmwares: Option<Vec<MinerFirmware>>,
) -> Result<Option<DeviceInfo>, Box<dyn Error>> {
    let search_makes = makes.unwrap_or(vec![
        MinerMake::AntMiner,
        MinerMake::WhatsMiner,
        MinerMake::AvalonMiner,
        MinerMake::EPic,
        MinerMake::Braiins,
        MinerMake::BitAxe,
    ]);
    let search_firmwares = firmwares.unwrap_or(vec![
        MinerFirmware::Stock,
        MinerFirmware::BraiinsOS,
        MinerFirmware::VNish,
        MinerFirmware::EPic,
        MinerFirmware::HiveOn,
        MinerFirmware::LuxOS,
        MinerFirmware::Marathon,
        MinerFirmware::MSKMiner,
    ]);
    let mut commands: HashSet<MinerCommand> = HashSet::new();

    for make in search_makes {
        for command in make.into_discovery_commands() {
            commands.insert(command);
        }
    }
    for firmware in search_firmwares {
        for command in firmware.into_discovery_commands() {
            commands.insert(command);
        }
    }

    for command in commands {
        match command {
            MinerCommand::RPC { command } => {
                let response = send_rpc_command(ip, command).await?;
                let (miner_type, miner_firmware) = parse_type_from_socket(response);
                dbg!(miner_type);
                dbg!(miner_firmware);
            }
            MinerCommand::WebAPI { command, https } => {
                let response = send_web_command(ip, command, https).await?;
                let (miner_type, miner_firmware) = parse_type_from_web(response);
                dbg!(miner_type);
                dbg!(miner_firmware);
            }
            _ => todo!(),
        }
    }
    Ok(None)
}

fn parse_type_from_socket(
    response: serde_json::Value,
) -> (Option<MinerMake>, Option<MinerFirmware>) {
    let json_string = response.to_string().to_uppercase();

    return if json_string.contains("BOSMINER") || json_string.contains("BOSER") {
        (None, Some(MinerFirmware::BraiinsOS))
    } else if json_string.contains("ANTMINER") && !json_string.contains("DEVDETAILS") {
        (Some(MinerMake::AntMiner), Some(MinerFirmware::Stock))
    } else {
        (None, None)
    };
}
fn parse_type_from_web(
    response: (String, HeaderMap, StatusCode),
) -> (Option<MinerMake>, Option<MinerFirmware>) {
    let (resp_text, resp_headers, resp_status) = response;

    let auth_header = match resp_headers.get("www-authenticate") {
        Some(header) => header.to_str().unwrap(),
        None => "",
    };
    return if resp_status == 401 && auth_header.contains("realm=\"antMiner") {
        (Some(MinerMake::AntMiner), Some(MinerFirmware::Stock))
    } else if resp_text.contains("Braiins OS") {
        (None, Some(MinerFirmware::BraiinsOS))
    } else {
        (None, None)
    };
}
