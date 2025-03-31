use std::net::IpAddr;

use asic_rs::get_miner;

#[tokio::main]
async fn main() {
    let miner_ip = IpAddr::from([192, 168, 86, 34]);

    let miner_info = get_miner(miner_ip).await.unwrap();
    dbg!(miner_info);
}
