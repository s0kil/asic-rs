use std::{fmt::Display, str::FromStr};

use antminer::AntMinerModel;
use whatsminer::WhatsMinerModel;

use super::MinerMake;

pub mod antminer;
pub mod whatsminer;

#[derive(Debug, Clone)]
pub struct ModelParseError;

impl Display for ModelParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse model")
    }
}

impl std::error::Error for ModelParseError {}

impl FromStr for WhatsMinerModel {
    type Err = ModelParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
            .map_err(|_| ModelParseError)
    }
}
impl FromStr for AntMinerModel {
    type Err = ModelParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
            .map_err(|_| ModelParseError)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MinerModel {
    AntMiner(AntMinerModel),
    WhatsMiner(WhatsMinerModel),
}

impl MinerModel {
    pub fn from_string(make: MinerMake, model_str: &str) -> Option<Self> {
        match make {
            MinerMake::AntMiner => {
                let model = AntMinerModel::from_str(model_str).ok();
                match model {
                    Some(model) => Some(MinerModel::AntMiner(model)),
                    None => None,
                }
            }
            MinerMake::WhatsMiner => {
                let model = WhatsMinerModel::from_str(model_str).ok();
                match model {
                    Some(model) => Some(MinerModel::WhatsMiner(model)),
                    None => None,
                }
            }
            _ => None,
        }
    }
}
