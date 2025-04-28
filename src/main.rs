use asic_rs::miners::api::rpc::luxminer::LUXMinerRPC;
use asic_rs::miners::api::rpc::traits::SendRPCCommand;
use serde_json::Value;
use std::net::IpAddr;

#[tokio::main]
async fn main() {
    let miner_ip = IpAddr::from([192, 168, 86, 34]);
    let port: u16 = 4028;

    let miner_rpc = LUXMinerRPC::new(miner_ip, Some(port));
    dbg!(miner_rpc.send_command::<Value>("devdetails").await);

    // let miner_info = get_miner(miner_ip).await.unwrap();
    // dbg!(miner_info);
}
