#[derive(Debug, PartialEq, Eq)]
pub enum MinerFirmware {
    Stock,
    BraiinsOS,
    VNish,
    EPic,
    HiveOn,
    LuxOS,
    Marathon,
    MSKMiner,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MinerMake {
    AntMiner,
    WhatsMiner,
    AvalonMiner,
    EPic,
    Braiins,
    BitAxe,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DeviceInfo {
    pub make: MinerMake,
    pub model: String, // for now
    pub firmware: MinerFirmware,
    pub algo: String,
}
