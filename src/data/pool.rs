pub enum PoolScheme {
    StratumV1,
    StratumV1SSL,
    StratumV2,
}

pub struct PoolURL {
    pub scheme: PoolScheme,
    pub host: String,
    pub port: u16,
    pub pubkey: Option<String>,
}

pub struct PoolData {
    pub position: u16,
    pub url: PoolURL,
    pub accepted_shares: u64,
    pub rejected_shares: u64,
    pub active: bool,
    pub alive: bool,
    pub user: String,
}
