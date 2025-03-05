use std::net::IpAddr;
use std::{collections::HashSet, error::Error};

use super::commands::{HTTP_WEB_ROOT, MinerCommand, RPC_VERSION};
use crate::data::device::{DeviceInfo, MinerFirmware, MinerMake};

use super::util::{parse_rpc_result, send_rpc_command};

pub(crate) trait IntoCommands {
    fn into_commands(&self) -> Vec<MinerCommand>;
}

impl IntoCommands for MinerMake {
    fn into_commands(&self) -> Vec<MinerCommand> {
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
impl IntoCommands for MinerFirmware {
    fn into_commands(&self) -> Vec<MinerCommand> {
        match self {
            MinerFirmware::Stock => vec![], // stock firmware needs miner make
            MinerFirmware::BraiinsOS => vec![],
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
        for command in make.into_commands() {
            commands.insert(command);
        }
    }
    for firmware in search_firmwares {
        for command in firmware.into_commands() {
            commands.insert(command);
        }
    }

    for command in commands {
        match command {
            MinerCommand::RPC { command } => {
                let response = send_rpc_command(ip, command).await?;
                dbg!(response);
            }
            MinerCommand::WebAPI { command, https } => {
                dbg!(&String::from("web command"));
            }
            _ => todo!(),
        }
    }
    Ok(None)
}
