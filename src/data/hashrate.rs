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

pub struct HashRate {
    pub value: f32,
    pub unit: HashRateUnit,
    pub algo: String,
}
