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

pub enum MinerMake {
    AntMiner,
    WhatsMiner,
    AvalonMiner,
    EPic,
    Braiins,
    BitAxe,
}

pub struct DeviceInfo {
    pub make: MinerMake,
    pub model: String, // for now
    pub firmware: MinerFirmware,
    pub algo: String,
}
