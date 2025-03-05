use std::{net::IpAddr, time::Duration};

use macaddr::MacAddr;
use measurements::{Power, Temperature};

use super::{
    board::BoardData, device::DeviceInfo, fan::FanData, hashrate::HashRate, message::MinerMessage,
    pool::PoolData,
};

pub struct MinerData {
    pub schema_version: String,
    pub timestamp: u32,
    pub ip: IpAddr,
    pub mac: MacAddr,
    pub device_info: DeviceInfo,
    pub serial_number: String,
    pub hostname: String,
    pub api_version: String,
    pub firmware_version: String,
    pub control_board_version: String,
    pub expected_hashboards: u8,
    pub hashboards: Vec<BoardData>,
    pub hashrate: HashRate,
    pub expected_chips: u16,
    pub total_chips: u16,
    pub expected_fans: u8,
    pub fans: Vec<FanData>,
    pub psu_fans: Vec<FanData>,
    pub average_temperature: Temperature,
    pub fluid_temperature: Temperature,
    pub wattage: Power,
    pub wattage_limit: Power,
    pub efficiency: f64,
    pub light_flashign: bool,
    pub messages: Vec<MinerMessage>,
    pub uptime: Duration,
    pub is_mining: bool,
    pub pools: Vec<PoolData>,
}
