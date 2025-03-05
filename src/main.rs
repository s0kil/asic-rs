use std::net::IpAddr;

use asic_rs::miners::factory::get_miner;

#[tokio::main]
async fn main() {
    let miner_ip = IpAddr::from([127, 0, 0, 1]);

    get_miner(&miner_ip, None, None).await.unwrap();
}
