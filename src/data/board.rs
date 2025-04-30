use super::hashrate::HashRate;
use measurements::{Frequency, Temperature, Voltage};

#[derive(Debug, Clone, PartialEq)]
pub struct ChipData {
    pub position: u16,
    pub hashrate: HashRate,
    pub temperature: Temperature,
    pub voltage: Voltage,
    pub frequency: Frequency,
    pub tuned: bool,
    pub working: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoardData {
    pub position: u16,
    pub hashrate: HashRate,
    pub expected_hashrate: HashRate,
    pub board_temperature: Temperature,
    pub intake_temperature: Temperature,
    pub outlet_temperature: Temperature,
    pub expected_chips: u16,
    pub working_chips: u16,
    pub serial_number: Option<String>,
    pub chips: Vec<ChipData>,
    pub voltage: Voltage,
    pub frequency: Frequency,
    pub tuned: bool,
    pub active: bool,
}
