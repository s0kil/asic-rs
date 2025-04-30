#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoolScheme {
    StratumV1,
    StratumV1SSL,
    StratumV2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoolURL {
    pub scheme: PoolScheme,
    pub host: String,
    pub port: u16,
    pub pubkey: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoolData {
    pub position: u16,
    pub url: PoolURL,
    pub accepted_shares: u64,
    pub rejected_shares: u64,
    pub active: bool,
    pub alive: bool,
    pub user: String,
}
