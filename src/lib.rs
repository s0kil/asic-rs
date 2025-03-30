use crate::data::device::models::MinerModel;
use crate::data::device::{MinerFirmware, MinerMake};
use crate::miners::factory::MinerFactory;
use std::error::Error;
use std::net::IpAddr;

pub mod data;
pub mod miners;

pub async fn get_miner(
    ip: IpAddr,
) -> Result<Option<(Option<MinerMake>, Option<MinerModel>, Option<MinerFirmware>)>, Box<dyn Error>>
{
    let factory = MinerFactory::new();
    factory.get_miner(ip).await
}
