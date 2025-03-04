pub enum PoolScheme {
    StratumV1,
    StratumV1SSL,
    StratumV2,
}

pub struct PoolURL {
    scheme: PoolScheme,
    host: String,
    port: u16,
    pubkey: Option<String>,
}

pub struct PoolData {
    position: u16,
    url: PoolURL,
    accepted_shares: u64,
    rejected_shares: u64,
    active: bool,
    alive: bool,
    user: String,
}
