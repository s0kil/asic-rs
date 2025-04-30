#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HashRateUnit {
    Hash,
    KiloHash,
    MegaHash,
    GigaHash,
    TeraHash,
    PetaHash,
    ExaHash,
    ZettaHash,
    YottaHash,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HashRate {
    pub value: f32,
    pub unit: HashRateUnit,
    pub algo: String,
}
