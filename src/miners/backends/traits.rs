use crate::data::miner::MinerData;
use async_trait::async_trait;

#[async_trait]
pub trait GetMinerData {
    async fn get_data(&self) -> MinerData;
}
