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
    value: f32,
    unit: HashRateUnit,
    algo: String,
}
