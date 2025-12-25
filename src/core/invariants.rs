/// Trait for entities that must never change once created.
/// Enforces the existence of an integrity check.
pub trait Immutable {
    fn integrity_hash(&self) -> &str;
}

/// Trait for entities that represent a derived, ideal state (Platonic).
pub trait Platonic {
    fn score(&self) -> f64;
    fn version(&self) -> u32;
}
