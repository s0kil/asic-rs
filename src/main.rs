use asic_rs::miners::api::rpc::btminer::BTMinerV3RPC;
use asic_rs::miners::api::rpc::luxminer::LUXMinerRPC;
use asic_rs::miners::api::rpc::traits::SendRPCCommand;
use serde_json::Value;
use std::net::IpAddr;

#[tokio::main]
async fn main() {
    let miner_ip = IpAddr::from([10, 0, 14, 208]);
    let port: u16 = 4433;

    let miner_rpc = BTMinerV3RPC::new(miner_ip, Some(port));
    dbg!(miner_rpc.send_command::<Value>("get.device.info").await);

    // let miner_info = get_miner(miner_ip).await.unwrap();
    // dbg!(miner_info);
}
