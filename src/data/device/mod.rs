use serde::{Deserialize, Serialize};

pub mod models;
pub use models::MinerModel;
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum MinerFirmware {
    #[serde(rename = "Stock")]
    Stock,
    #[serde(rename = "BraiinsOS")]
    BraiinsOS,
    #[serde(rename = "VNish")]
    VNish,
    #[serde(rename = "ePIC")]
    EPic,
    #[serde(rename = "HiveOS")]
    HiveOS,
    #[serde(rename = "LuxOS")]
    LuxOS,
    #[serde(rename = "Marathon")]
    Marathon,
    #[serde(rename = "MSKMiner")]
    MSKMiner,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum MinerMake {
    #[serde(rename = "AntMiner")]
    AntMiner,
    #[serde(rename = "WhatsMiner")]
    WhatsMiner,
    #[serde(rename = "AvalonMiner")]
    AvalonMiner,
    #[serde(rename = "ePIC")]
    EPic,
    #[serde(rename = "Braiins")]
    Braiins,
    #[serde(rename = "BitAxe")]
    BitAxe,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum HashAlgorithm {
    #[serde(rename = "SHA256")]
    SHA256,
    #[serde(rename = "Scrypt")]
    Scrypt,
    #[serde(rename = "X11")]
    X11,
    #[serde(rename = "Blake2S256")]
    Blake2S256,
    #[serde(rename = "Kadena")]
    Kadena,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DeviceInfo {
    pub make: MinerMake,
    pub model: MinerModel,
    pub firmware: MinerFirmware,
    pub algo: HashAlgorithm,
}
