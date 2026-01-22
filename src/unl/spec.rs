//! UNL Specification - Rules and Invariants

use serde::{Deserialize, Serialize};

/// UNL Specification container.
#[derive(Debug, Clone)]
pub struct UnlSpec {
    pub version: (u8, u8, u8),
    pub rules: Vec<UnlRule>,
    pub invariants: Vec<UnlInvariant>,
}

/// A single UNL rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlRule {
    pub id: String,
    pub description: String,
    pub mandatory: bool,
}

/// An invariant that must always hold.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlInvariant {
    pub id: String,
    pub axiom: String,
}

impl UnlSpec {
    /// Creates the canonical UNL v1.0.0 specification.
    pub fn v1() -> Self {
        Self {
            version: (1, 0, 0),
            rules: vec![
                UnlRule {
                    id: "R001".into(),
                    description: "Every stimulus has discrete base frequency".into(),
                    mandatory: true,
                },
                UnlRule {
                    id: "R002".into(),
                    description: "Amplitude is continuous ∈ ℝ[0,1]".into(),
                    mandatory: true,
                },
                UnlRule {
                    id: "R003".into(),
                    description: "BOF/EOF delimit valid transmissions".into(),
                    mandatory: true,
                },
                UnlRule {
                    id: "R004".into(),
                    description: "Motor outputs are vectors ℝⁿ".into(),
                    mandatory: true,
                },
            ],
            invariants: vec![
                UnlInvariant {
                    id: "I001".into(),
                    axiom: "CP = M_P × M_N × M_C × M_M".into(),
                },
                UnlInvariant {
                    id: "I002".into(),
                    axiom: "∀ motor: score ∈ [0,1]".into(),
                },
                UnlInvariant {
                    id: "I003".into(),
                    axiom: "motor < VETO_THRESHOLD ⟹ CP = 0".into(),
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unl_v1_has_rules() {
        let spec = UnlSpec::v1();
        assert!(!spec.rules.is_empty());
        assert!(!spec.invariants.is_empty());
    }
}
