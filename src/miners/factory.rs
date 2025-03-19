use futures::future::FutureExt;
use futures::pin_mut;
use std::net::IpAddr;
use std::time::Duration;
use std::{collections::HashSet, error::Error};

use diqwest::WithDigestAuth;
use reqwest::header::HeaderMap;
use reqwest::{Client, Response, StatusCode};
use tokio::task::JoinSet;

use super::commands::{HTTP_WEB_ROOT, HTTPS_WEB_ROOT, MinerCommand, RPC_DEVDETAILS, RPC_VERSION};
use crate::data::device::models::MinerModel;
use crate::data::device::{MinerFirmware, MinerMake};

use super::util::{send_rpc_command, send_web_command};

const MAX_WAIT_TIME: Duration = Duration::from_secs(5);

pub(crate) trait DiscoveryCommands {
    fn get_discovery_commands(&self) -> Vec<MinerCommand>;
}
pub(crate) trait ModelSelection {
    async fn get_model(&self, ip: IpAddr) -> Option<MinerModel>;
}

impl DiscoveryCommands for MinerMake {
    fn get_discovery_commands(&self) -> Vec<MinerCommand> {
        match self {
            MinerMake::AntMiner => vec![RPC_VERSION, HTTP_WEB_ROOT],
            MinerMake::WhatsMiner => vec![RPC_DEVDETAILS, HTTPS_WEB_ROOT],
            MinerMake::AvalonMiner => vec![RPC_VERSION, HTTP_WEB_ROOT],
            MinerMake::EPic => vec![HTTP_WEB_ROOT],
            MinerMake::Braiins => vec![RPC_VERSION, HTTP_WEB_ROOT],
            MinerMake::BitAxe => vec![HTTP_WEB_ROOT],
        }
    }
}
impl DiscoveryCommands for MinerFirmware {
    fn get_discovery_commands(&self) -> Vec<MinerCommand> {
        match self {
            MinerFirmware::Stock => vec![], // stock firmware needs miner make
            MinerFirmware::BraiinsOS => vec![RPC_VERSION, HTTP_WEB_ROOT],
            MinerFirmware::VNish => vec![HTTP_WEB_ROOT, RPC_VERSION],
            MinerFirmware::EPic => vec![HTTP_WEB_ROOT],
            MinerFirmware::HiveOS => vec![],
            MinerFirmware::LuxOS => vec![],
            MinerFirmware::Marathon => vec![],
            MinerFirmware::MSKMiner => vec![],
        }
    }
}
impl ModelSelection for MinerFirmware {
    async fn get_model(&self, ip: IpAddr) -> Option<MinerModel> {
        match self {
            _ => None,
        }
    }
}

impl ModelSelection for MinerMake {
    async fn get_model(&self, ip: IpAddr) -> Option<MinerModel> {
        match self {
            MinerMake::AntMiner => get_model_antminer(ip).await,
            MinerMake::WhatsMiner => get_model_whatsminer(ip).await,
            _ => None,
        }
    }
}

pub async fn get_miner(
    ip: IpAddr,
    makes: Option<Vec<MinerMake>>,
    firmwares: Option<Vec<MinerFirmware>>,
) -> Result<Option<(Option<MinerMake>, Option<MinerModel>, Option<MinerFirmware>)>, Box<dyn Error>>
{
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
        MinerFirmware::HiveOS,
        MinerFirmware::LuxOS,
        MinerFirmware::Marathon,
        MinerFirmware::MSKMiner,
    ]);
    let mut commands: HashSet<MinerCommand> = HashSet::new();

    for make in search_makes {
        for command in make.get_discovery_commands() {
            commands.insert(command);
        }
    }
    for firmware in search_firmwares {
        for command in firmware.get_discovery_commands() {
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

    match miner_info {
        Some((make, firmware)) => match (make, firmware) {
            (Some(make), Some(MinerFirmware::Stock)) => {
                let model = make.get_model(ip).await;
                Ok(Some((Some(make), model, firmware)))
            }
            _ => Ok(None),
        },
        None => Ok(None),
    }
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
        _ if json_string.contains("AVALON") => {
            Some((Some(MinerMake::AvalonMiner), Some(MinerFirmware::Stock)))
        }
        _ if json_string.contains("VNISH") => Some((None, Some(MinerFirmware::VNish))),
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

    match () {
        _ if resp_status == 401 && auth_header.contains("realm=\"antMiner") => {
            Some((Some(MinerMake::AntMiner), Some(MinerFirmware::Stock)))
        }
        _ if resp_text.contains("Braiins OS") => Some((None, Some(MinerFirmware::BraiinsOS))),
        _ if resp_text.contains("AxeOS") => {
            Some((Some(MinerMake::BitAxe), Some(MinerFirmware::Stock)))
        }
        _ if resp_text.contains("Miner Web Dashboard") => Some((None, Some(MinerFirmware::EPic))),
        _ if resp_text.contains("Avalon") => {
            Some((Some(MinerMake::AvalonMiner), Some(MinerFirmware::Stock)))
        }
        _ if resp_text.contains("AnthillOS") => Some((None, Some(MinerFirmware::VNish))),
        _ if redirect_header.contains("https://") && resp_status == 307
            || resp_text.contains("/cgi-bin/luci") =>
        {
            Some((Some(MinerMake::WhatsMiner), Some(MinerFirmware::Stock)))
        }
        _ => None,
    }
}

async fn get_model_antminer(ip: IpAddr) -> Option<MinerModel> {
    let response: Option<Response> = Client::new()
        .get(format!("http://{}/cgi-bin/get_system_info.cgi", ip))
        .send_with_digest_auth("root", "root")
        .await
        .ok();
    match response {
        Some(data) => {
            let json_data = data.json::<serde_json::Value>().await.ok()?;
            MinerModel::from_string(
                MinerMake::AntMiner,
                &json_data["minertype"].as_str().unwrap_or("").to_uppercase(),
            )
        }
        None => None,
    }
}
async fn get_model_whatsminer(ip: IpAddr) -> Option<MinerModel> {
    let response = send_rpc_command(&ip, "devdetails").await;
    match response {
        Some(json_data) => {
            let model = json_data["DEVDETAILS"][0]["Model"].as_str();
            if model.is_none() {
                return None;
            }
            let mut model = model.unwrap().replace("_", "");
            model.pop();
            model.push('0');

            MinerModel::from_string(MinerMake::WhatsMiner, &model)
        }
        None => None,
    }
}
