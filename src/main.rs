use asic_rs::get_miner;
use asic_rs::miners::backends::traits::GetMinerData;
use std::net::IpAddr;

#[tokio::main]
async fn main() {
    let miner_ip: IpAddr = "192.13.1.246".parse().unwrap();

    let miner = get_miner(miner_ip).await.unwrap();
    if miner.is_some() {
        dbg!(miner.unwrap().get_data().await);
    }
}
