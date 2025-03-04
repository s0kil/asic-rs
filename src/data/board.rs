use super::hashrate::HashRate;
use measurements::{Frequency, Temperature, Voltage};

pub struct ChipData {
    position: u16,
    hashrate: HashRate,
    temperature: Temperature,
    voltage: Voltage,
    frequency: Frequency,
    tuned: bool,
    working: bool,
}

pub struct BoardData {
    position: u16,
    hashrate: HashRate,
    expected_hashrate: HashRate,
    board_temperature: Temperature,
    intake_temperature: Temperature,
    outlet_temperature: Temperature,
    expected_chips: u16,
    working_chips: u16,
    serial_number: Option<String>,
    chips: Vec<ChipData>,
    voltage: Voltage,
    frequency: Frequency,
    tuned: bool,
    active: bool,
}
