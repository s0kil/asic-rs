use std::{hash::Hash, net::IpAddr, time::Duration};

use macaddr::MacAddr;
use measurements::{Power, Temperature};

use super::{
    board::BoardData, device::DeviceInfo, fan::FanData, hashrate::HashRate, message::MinerMessage,
    pool::PoolData,
};

struct MinerData {
    schema_version: String,
    timestamp: u32,
    ip: IpAddr,
    mac: MacAddr,
    device_info: DeviceInfo,
    serial_number: String,
    hostname: String,
    api_version: String,
    firmware_version: String,
    control_board_version: String,
    expected_hashboards: u8,
    hashboards: Vec<BoardData>,
    hashrate: HashRate,
    expected_chips: u16,
    total_chips: u16,
    expected_fans: u8,
    fans: Vec<FanData>,
    psu_fans: Vec<FanData>,
    average_temperature: Temperature,
    fluid_temperature: Temperature,
    wattage: Power,
    wattage_limit: Power,
    efficiency: f64,
    light_flashign: bool,
    messages: Vec<MinerMessage>,
    uptime: Duration,
    is_mining: bool,
    pools: Vec<PoolData>,
}
