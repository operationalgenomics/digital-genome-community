// Teste de Stress de Colisão - v1.0.2
// Geração de 10.000 IDs determinísticos distintos
// Validação: ZERO colisões

#[cfg(test)]
mod collision_stress_tests {
    use crate::core_types::CanonicalId;
    use crate::coordination::vibration::{VibrationId, WorkId};
    use crate::coordination::fragmentation::WorkerId;
    use crate::coordination::manifestation_delta::GdcId;
    use crate::core_types::identifiers::{ActionId, DnaId, SynapseId, NeuronId, BrainId};
    use crate::coordination::vibration::{StructuralRequirements, BudgetDelta};
    use std::collections::HashSet;

    const STRESS_TEST_COUNT: usize = 10_000;

    #[test]
    fn test_canonical_id_collision_stress_10k() {
        let mut ids = HashSet::new();
        
        for i in 0..STRESS_TEST_COUNT {
            let payload = format!("payload-{}", i);
            let id = CanonicalId::from_context("stress-test", payload.as_bytes(), i as u64);
            
            assert!(ids.insert(id), "Colisão detectada no índice {}", i);
        }
        
        assert_eq!(ids.len(), STRESS_TEST_COUNT, "Todos os IDs devem ser únicos");
    }

    #[test]
    fn test_action_id_collision_stress_10k() {
        let mut ids = HashSet::new();
        
        for i in 0..STRESS_TEST_COUNT {
            let seed = format!("action-{}", i);
            let id = ActionId::new_deterministic(seed.as_bytes());
            
            assert!(ids.insert(id), "Colisão de ActionId no índice {}", i);
        }
        
        assert_eq!(ids.len(), STRESS_TEST_COUNT);
    }

    #[test]
    fn test_dna_id_collision_stress_10k() {
        let mut ids = HashSet::new();
        
        for i in 0..STRESS_TEST_COUNT {
            let seed = format!("dna-{}", i);
            let id = DnaId::new_deterministic(seed.as_bytes());
            
            assert!(ids.insert(id), "Colisão de DnaId no índice {}", i);
        }
        
        assert_eq!(ids.len(), STRESS_TEST_COUNT);
    }

    #[test]
    fn test_vibration_id_collision_stress_10k() {
        let mut ids = HashSet::new();
        let req = StructuralRequirements::basic();
        let budget = BudgetDelta::default();
        
        for i in 0..STRESS_TEST_COUNT {
            let id = VibrationId::from_vibration_data(&req, &budget, i as u64);
            
            assert!(ids.insert(id), "Colisão de VibrationId no índice {}", i);
        }
        
        assert_eq!(ids.len(), STRESS_TEST_COUNT);
    }

    #[test]
    fn test_work_id_collision_stress_10k() {
        let mut ids = HashSet::new();
        
        for i in 0..STRESS_TEST_COUNT {
            let stimulus = format!("stimulus-{}", i);
            let id = WorkId::from_stimulus(stimulus.as_bytes(), 0);
            
            assert!(ids.insert(id), "Colisão de WorkId no índice {}", i);
        }
        
        assert_eq!(ids.len(), STRESS_TEST_COUNT);
    }

    #[test]
    fn test_worker_id_collision_stress_10k() {
        let mut ids = HashSet::new();
        
        for i in 0..STRESS_TEST_COUNT {
            let id = WorkerId::from_worker_index(i, 0);
            
            assert!(ids.insert(id), "Colisão de WorkerId no índice {}", i);
        }
        
        assert_eq!(ids.len(), STRESS_TEST_COUNT);
    }

    #[test]
    fn test_gdc_id_collision_stress_10k() {
        let mut ids = HashSet::new();
        
        for i in 0..STRESS_TEST_COUNT {
            let hash = format!("gdc-state-{}", i);
            let id = GdcId::from_state_hash(hash.as_bytes(), 0);
            
            assert!(ids.insert(id), "Colisão de GdcId no índice {}", i);
        }
        
        assert_eq!(ids.len(), STRESS_TEST_COUNT);
    }

    #[test]
    fn test_neuron_id_collision_stress_10k() {
        let mut ids = HashSet::new();
        
        for i in 0..STRESS_TEST_COUNT {
            let seed = format!("neuron-{}", i);
            let id = NeuronId::new_deterministic(seed.as_bytes());
            
            assert!(ids.insert(id), "Colisão de NeuronId no índice {}", i);
        }
        
        assert_eq!(ids.len(), STRESS_TEST_COUNT);
    }

    #[test]
    fn test_synapse_id_collision_stress_10k() {
        let mut ids = HashSet::new();
        
        for i in 0..STRESS_TEST_COUNT {
            let seed = format!("synapse-{}", i);
            let id = SynapseId::new_deterministic(seed.as_bytes());
            
            assert!(ids.insert(id), "Colisão de SynapseId no índice {}", i);
        }
        
        assert_eq!(ids.len(), STRESS_TEST_COUNT);
    }

    #[test]
    fn test_brain_id_collision_stress_10k() {
        let mut ids = HashSet::new();
        
        for i in 0..STRESS_TEST_COUNT {
            let seed = format!("brain-{}", i);
            let id = BrainId::new_deterministic(seed.as_bytes());
            
            assert!(ids.insert(id), "Colisão de BrainId no índice {}", i);
        }
        
        assert_eq!(ids.len(), STRESS_TEST_COUNT);
    }
}
