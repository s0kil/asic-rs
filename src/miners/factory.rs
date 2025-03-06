use futures::future::FutureExt;
use futures::pin_mut;
use std::net::IpAddr;
use std::time::Duration;
use std::{collections::HashSet, error::Error};

use reqwest::StatusCode;
use reqwest::header::HeaderMap;
use tokio::task::JoinSet;

use super::commands::{HTTP_WEB_ROOT, HTTPS_WEB_ROOT, MinerCommand, RPC_DEVDETAILS, RPC_VERSION};
use crate::data::device::{MinerFirmware, MinerMake};

use super::util::{send_rpc_command, send_web_command};

const MAX_WAIT_TIME: Duration = Duration::from_secs(5);

pub(crate) trait DiscoveryCommands {
    fn into_discovery_commands(&self) -> Vec<MinerCommand>;
}

impl DiscoveryCommands for MinerMake {
    fn into_discovery_commands(&self) -> Vec<MinerCommand> {
        match self {
            MinerMake::AntMiner => vec![RPC_VERSION, HTTP_WEB_ROOT],
            MinerMake::WhatsMiner => vec![RPC_DEVDETAILS, HTTPS_WEB_ROOT],
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
    ip: IpAddr,
    makes: Option<Vec<MinerMake>>,
    firmwares: Option<Vec<MinerFirmware>>,
) -> Result<Option<(Option<MinerMake>, Option<MinerFirmware>)>, Box<dyn Error>> {
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

    let mut discovery_tasks = JoinSet::new();
    for command in commands {
        let _ = discovery_tasks.spawn(get_miner_type_from_command(ip, command));
    }

    let timeout = tokio::time::sleep(MAX_WAIT_TIME).fuse();
    let tasks = tokio::spawn(async move {
        loop {
            if discovery_tasks.is_empty() {
                return None;
            };
            match discovery_tasks.join_next().await.unwrap_or(Ok(None)) {
                Ok(Some(result)) => {
                    return Some(result);
                }
                _ => continue,
            };
        }
    });

    pin_mut!(timeout, tasks);

    let miner_info = tokio::select!(
        Ok(miner_info) = &mut tasks => {
            miner_info
        },
        _ = &mut timeout => {
            None
        }
    );
    Ok(miner_info)
}

async fn get_miner_type_from_command(
    ip: IpAddr,
    command: MinerCommand,
) -> Option<(Option<MinerMake>, Option<MinerFirmware>)> {
    match command {
        MinerCommand::RPC { command } => {
            let response = send_rpc_command(&ip, command).await?;
            parse_type_from_socket(response)
        }
        MinerCommand::WebAPI { command, https } => {
            let response = send_web_command(&ip, command, https).await?;
            parse_type_from_web(response)
        }
        _ => None,
    }
}

fn parse_type_from_socket(
    response: serde_json::Value,
) -> Option<(Option<MinerMake>, Option<MinerFirmware>)> {
    let json_string = response.to_string().to_uppercase();

    match () {
        _ if json_string.contains("BOSMINER") || json_string.contains("BOSER") => {
            Some((None, Some(MinerFirmware::BraiinsOS)))
        }
        _ if json_string.contains("BITMICRO") || json_string.contains("BTMINER") => {
            Some((Some(MinerMake::WhatsMiner), Some(MinerFirmware::Stock)))
        }
        _ if json_string.contains("ANTMINER") && !json_string.contains("DEVDETAILS") => {
            Some((Some(MinerMake::AntMiner), Some(MinerFirmware::Stock)))
        }
        _ => None,
    }
}

fn parse_type_from_web(
    response: (String, HeaderMap, StatusCode),
) -> Option<(Option<MinerMake>, Option<MinerFirmware>)> {
    let (resp_text, resp_headers, resp_status) = response;
    let auth_header = match resp_headers.get("www-authenticate") {
        Some(header) => header.to_str().unwrap(),
        None => "",
    };
    let redirect_header = match resp_headers.get("location") {
        Some(header) => header.to_str().unwrap(),
        None => "",
    };

    if resp_status == 401 && auth_header.contains("realm=\"antMiner") {
        Some((Some(MinerMake::AntMiner), Some(MinerFirmware::Stock)))
    } else if resp_text.contains("Braiins OS") {
        Some((None, Some(MinerFirmware::BraiinsOS)))
    } else if redirect_header.contains("https://") && resp_status == 307
        || resp_text.contains("/cgi-bin/luci")
    {
        Some((Some(MinerMake::WhatsMiner), Some(MinerFirmware::Stock)))
    } else {
        None
    }
}
