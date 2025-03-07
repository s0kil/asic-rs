use std::net::IpAddr;

use asic_rs::miners::factory::get_miner;

#[tokio::main]
async fn main() {
    let miner_ip = IpAddr::from([192, 168, 86, 21]);

    let miner_info = get_miner(miner_ip, None, None).await.unwrap();
    dbg!(miner_info);
}
