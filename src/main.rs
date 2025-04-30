use asic_rs::miners::api::rpc::btminer::BTMinerV3RPC;
use asic_rs::miners::api::rpc::luxminer::LUXMinerRPC;
use asic_rs::miners::api::rpc::traits::SendRPCCommand;
use asic_rs::miners::backends::btminer::BTMinerV3Backend;
use serde_json::Value;
use std::net::IpAddr;

#[tokio::main]
async fn main() {
    let miner_ip = IpAddr::from([10, 0, 14, 208]);

    let miner = BTMinerV3Backend::new(miner_ip);
    dbg!(miner.get_miner_status_summary().await.unwrap());

    // let miner = BTMinerV3Backend::new(miner_ip);
    // dbg!(miner.get_device_info().await.unwrap());

    // let miner_info = get_miner(miner_ip).await.unwrap();
    // dbg!(miner_info);
}
