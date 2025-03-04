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
    make: MinerMake,
    model: String, // for now
    firmware: MinerFirmware,
    algo: String,
}
