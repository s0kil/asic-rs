use crate::data::miner::MinerData;

pub trait GetMinerData: Send {
    fn get_data(&self) -> impl std::future::Future<Output = MinerData> + Send;
}
