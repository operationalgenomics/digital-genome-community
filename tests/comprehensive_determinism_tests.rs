// Testes Abrangentes de Determinismo - v1.0.2
// Validação: Mesmo input → Mesmo output (SEMPRE)

#[cfg(test)]
mod comprehensive_determinism_tests {
    use super::*;
    use crate::core_types::CanonicalId;
    use crate::coordination::vibration::{VibrationId, WorkId, StructuralRequirements, BudgetDelta};
    use crate::coordination::fragmentation::WorkerId;
    use crate::coordination::manifestation_delta::GdcId;
    use crate::core_types::identifiers::{ActionId, DnaId, SynapseId, NeuronId, BrainId};

    #[test]
    fn test_canonical_id_determinism_comprehensive() {
        // Teste 1: Mesmos parâmetros → Mesmo ID
        for _ in 0..100 {
            let id1 = CanonicalId::from_context("test", b"payload", 42);
            let id2 = CanonicalId::from_context("test", b"payload", 42);
            assert_eq!(id1, id2, "CanonicalId deve ser determinístico");
        }
        
        // Teste 2: Parâmetros diferentes → IDs diferentes
        let id_a = CanonicalId::from_context("test", b"payload1", 42);
        let id_b = CanonicalId::from_context("test", b"payload2", 42);
        let id_c = CanonicalId::from_context("test", b"payload", 43);
        
        assert_ne!(id_a, id_b, "Payloads diferentes devem gerar IDs diferentes");
        assert_ne!(id_a, id_c, "Sequences diferentes devem gerar IDs diferentes");
    }

    #[test]
    fn test_vibration_id_determinism_comprehensive() {
        let req = StructuralRequirements::basic();
        let budget = BudgetDelta::default();
        
        // Mesmos parâmetros → Mesmo ID (100 vezes)
        for _ in 0..100 {
            let id1 = VibrationId::from_vibration_data(&req, &budget, 0);
            let id2 = VibrationId::from_vibration_data(&req, &budget, 0);
            assert_eq!(id1, id2, "VibrationId deve ser determinístico");
        }
        
        // Sequences diferentes → IDs diferentes
        let id_seq0 = VibrationId::from_vibration_data(&req, &budget, 0);
        let id_seq1 = VibrationId::from_vibration_data(&req, &budget, 1);
        assert_ne!(id_seq0, id_seq1, "Sequences diferentes devem gerar IDs diferentes");
    }

    #[test]
    fn test_work_id_determinism_comprehensive() {
        // Mesmo estímulo → Mesmo ID (100 vezes)
        for _ in 0..100 {
            let id1 = WorkId::from_stimulus(b"test-stimulus", 0);
            let id2 = WorkId::from_stimulus(b"test-stimulus", 0);
            assert_eq!(id1, id2, "WorkId deve ser determinístico");
        }
        
        // Estímulos diferentes → IDs diferentes
        let id_a = WorkId::from_stimulus(b"stimulus-a", 0);
        let id_b = WorkId::from_stimulus(b"stimulus-b", 0);
        assert_ne!(id_a, id_b, "Estímulos diferentes devem gerar IDs diferentes");
    }

    #[test]
    fn test_worker_id_determinism_comprehensive() {
        // Mesmo índice → Mesmo ID (100 vezes)
        for _ in 0..100 {
            let id1 = WorkerId::from_worker_index(42, 0);
            let id2 = WorkerId::from_worker_index(42, 0);
            assert_eq!(id1, id2, "WorkerId deve ser determinístico");
        }
        
        // Índices diferentes → IDs diferentes
        let id_a = WorkerId::from_worker_index(1, 0);
        let id_b = WorkerId::from_worker_index(2, 0);
        assert_ne!(id_a, id_b, "Índices diferentes devem gerar IDs diferentes");
    }

    #[test]
    fn test_gdc_id_determinism_comprehensive() {
        // Mesmo hash → Mesmo ID (100 vezes)
        for _ in 0..100 {
            let id1 = GdcId::from_state_hash(b"state-hash", 0);
            let id2 = GdcId::from_state_hash(b"state-hash", 0);
            assert_eq!(id1, id2, "GdcId deve ser determinístico");
        }
        
        // Hashes diferentes → IDs diferentes
        let id_a = GdcId::from_state_hash(b"hash-a", 0);
        let id_b = GdcId::from_state_hash(b"hash-b", 0);
        assert_ne!(id_a, id_b, "Hashes diferentes devem gerar IDs diferentes");
    }

    #[test]
    fn test_action_id_determinism_comprehensive() {
        // Mesma seed → Mesmo ID (100 vezes)
        for _ in 0..100 {
            let id1 = ActionId::new_deterministic(b"action-seed");
            let id2 = ActionId::new_deterministic(b"action-seed");
            assert_eq!(id1, id2, "ActionId deve ser determinístico");
        }
        
        // Seeds diferentes → IDs diferentes
        let id_a = ActionId::new_deterministic(b"seed-a");
        let id_b = ActionId::new_deterministic(b"seed-b");
        assert_ne!(id_a, id_b, "Seeds diferentes devem gerar IDs diferentes");
    }

    #[test]
    fn test_dna_id_determinism_comprehensive() {
        for _ in 0..100 {
            let id1 = DnaId::new_deterministic(b"dna-seed");
            let id2 = DnaId::new_deterministic(b"dna-seed");
            assert_eq!(id1, id2, "DnaId deve ser determinístico");
        }
    }

    #[test]
    fn test_synapse_id_determinism_comprehensive() {
        for _ in 0..100 {
            let id1 = SynapseId::new_deterministic(b"synapse-seed");
            let id2 = SynapseId::new_deterministic(b"synapse-seed");
            assert_eq!(id1, id2, "SynapseId deve ser determinístico");
        }
    }

    #[test]
    fn test_neuron_id_determinism_comprehensive() {
        for _ in 0..100 {
            let id1 = NeuronId::new_deterministic(b"neuron-seed");
            let id2 = NeuronId::new_deterministic(b"neuron-seed");
            assert_eq!(id1, id2, "NeuronId deve ser determinístico");
        }
    }

    #[test]
    fn test_brain_id_determinism_comprehensive() {
        for _ in 0..100 {
            let id1 = BrainId::new_deterministic(b"brain-seed");
            let id2 = BrainId::new_deterministic(b"brain-seed");
            assert_eq!(id1, id2, "BrainId deve ser determinístico");
        }
    }

    #[test]
    fn test_cross_instance_determinism() {
        // Validar que determinismo funciona entre "instâncias" diferentes
        let ids1: Vec<_> = (0..10)
            .map(|i| ActionId::new_deterministic(format!("action-{}", i).as_bytes()))
            .collect();
        
        let ids2: Vec<_> = (0..10)
            .map(|i| ActionId::new_deterministic(format!("action-{}", i).as_bytes()))
            .collect();
        
        assert_eq!(ids1, ids2, "Sequências devem ser idênticas");
    }

    #[test]
    fn test_id_independence() {
        // Mesmo seed em tipos diferentes deve gerar IDs diferentes
        let seed = b"same-seed";
        
        let action = ActionId::new_deterministic(seed);
        let dna = DnaId::new_deterministic(seed);
        let synapse = SynapseId::new_deterministic(seed);
        
        // Não podemos comparar tipos diferentes diretamente,
        // mas podemos verificar que UUIDs internos são diferentes
        assert_ne!(action.as_uuid(), dna.as_uuid(), 
                   "Tipos diferentes com mesma seed devem gerar UUIDs diferentes");
        assert_ne!(action.as_uuid(), synapse.as_uuid(), 
                   "Tipos diferentes com mesma seed devem gerar UUIDs diferentes");
    }
}
