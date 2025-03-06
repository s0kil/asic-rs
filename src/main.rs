use std::net::IpAddr;

use asic_rs::miners::factory::get_miner;

#[tokio::main]
async fn main() {
    let miner_ip = IpAddr::from([10, 0, 1, 82]);

    get_miner(&miner_ip, None, None).await.unwrap();
}
