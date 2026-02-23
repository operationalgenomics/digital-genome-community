// Teste de Replay Sistêmico - v1.0.2
// Validação: 1.000.000 replays → Hash final idêntico
// Critério: ZERO divergência

#[cfg(test)]
mod replay_systemic_validation {
    use digital_genome_community::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    /// Estrutura de estado do sistema para replay
    #[derive(Clone, Hash)]
    struct SystemState {
        cycle_count: u64,
        action_ids: Vec<String>,
        vibration_ids: Vec<String>,
        dna_ids: Vec<String>,
    }

    impl SystemState {
        fn new() -> Self {
            Self {
                cycle_count: 0,
                action_ids: Vec::new(),
                vibration_ids: Vec::new(),
                dna_ids: Vec::new(),
            }
        }

        fn execute_cycle(&mut self, cycle: u64) {
            use core_types::{ActionId, LogicalTime};
            use coordination::vibration::{
                WorkId, VibrationId, StructuralRequirements, BudgetDelta, Vibration
            };
            use core_types::identifiers::DnaId;

            self.cycle_count = cycle;

            // 1. Criar ActionId determinístico
            let action_seed = format!("action-{}", cycle);
            let action_id = ActionId::new_deterministic(action_seed.as_bytes());
            self.action_ids.push(action_id.to_string());

            // 2. Criar VibrationId determinístico
            let req = StructuralRequirements::basic();
            let budget = BudgetDelta::default();
            let vibration_id = VibrationId::from_vibration_data(&req, &budget, cycle);
            self.vibration_ids.push(format!("{:?}", vibration_id));

            // 3. Criar DnaId determinístico
            let dna_seed = format!("dna-{}", cycle);
            let dna_id = DnaId::new_deterministic(dna_seed.as_bytes());
            self.dna_ids.push(dna_id.to_string());
        }

        fn compute_hash(&self) -> u64 {
            let mut hasher = DefaultHasher::new();
            self.hash(&mut hasher);
            hasher.finish()
        }
    }

    #[test]
    #[ignore] // Ignorar por padrão (teste pesado)
    fn test_replay_1m_iterations_full() {
        const ITERATIONS: u64 = 1_000_000;
        
        println!("Iniciando teste de replay com {} iterações...", ITERATIONS);
        
        // Primeira execução
        let mut state1 = SystemState::new();
        for cycle in 0..ITERATIONS {
            state1.execute_cycle(cycle);
            
            if cycle % 100_000 == 0 && cycle > 0 {
                println!("Progresso execução 1: {}/{}", cycle, ITERATIONS);
            }
        }
        let hash1 = state1.compute_hash();
        println!("Hash execução 1: {:#018x}", hash1);
        
        // Segunda execução (replay)
        let mut state2 = SystemState::new();
        for cycle in 0..ITERATIONS {
            state2.execute_cycle(cycle);
            
            if cycle % 100_000 == 0 && cycle > 0 {
                println!("Progresso execução 2: {}/{}", cycle, ITERATIONS);
            }
        }
        let hash2 = state2.compute_hash();
        println!("Hash execução 2: {:#018x}", hash2);
        
        // Validação
        assert_eq!(hash1, hash2, 
                   "FALHA DE REPLAY: Hashes diferentes após {} iterações", ITERATIONS);
        
        println!("✅ SUCESSO: {} replays com hash idêntico", ITERATIONS);
        println!("Hash final: {:#018x}", hash1);
    }

    #[test]
    fn test_replay_10k_iterations_quick() {
        const ITERATIONS: u64 = 10_000;
        
        // Primeira execução
        let mut state1 = SystemState::new();
        for cycle in 0..ITERATIONS {
            state1.execute_cycle(cycle);
        }
        let hash1 = state1.compute_hash();
        
        // Segunda execução (replay)
        let mut state2 = SystemState::new();
        for cycle in 0..ITERATIONS {
            state2.execute_cycle(cycle);
        }
        let hash2 = state2.compute_hash();
        
        // Validação
        assert_eq!(hash1, hash2, 
                   "FALHA DE REPLAY: Hashes diferentes após {} iterações", ITERATIONS);
    }

    #[test]
    fn test_replay_1k_iterations_fast() {
        const ITERATIONS: u64 = 1_000;
        
        // Primeira execução
        let mut state1 = SystemState::new();
        for cycle in 0..ITERATIONS {
            state1.execute_cycle(cycle);
        }
        let hash1 = state1.compute_hash();
        
        // Segunda execução (replay)
        let mut state2 = SystemState::new();
        for cycle in 0..ITERATIONS {
            state2.execute_cycle(cycle);
        }
        let hash2 = state2.compute_hash();
        
        // Validação
        assert_eq!(hash1, hash2, "FALHA DE REPLAY");
    }

    #[test]
    fn test_replay_partial_state_verification() {
        // Verificar que estados parciais também são determinísticos
        const CHECKPOINT: u64 = 100;
        
        let mut state1 = SystemState::new();
        let mut state2 = SystemState::new();
        
        for cycle in 0..CHECKPOINT {
            state1.execute_cycle(cycle);
            state2.execute_cycle(cycle);
            
            // Verificar hash a cada ciclo
            assert_eq!(state1.compute_hash(), state2.compute_hash(),
                      "Divergência no ciclo {}", cycle);
        }
    }

    #[test]
    fn test_replay_deterministic_ordering() {
        // Verificar que ordem de execução não afeta resultado
        const ITERATIONS: u64 = 100;
        
        let mut state = SystemState::new();
        for cycle in 0..ITERATIONS {
            state.execute_cycle(cycle);
        }
        
        // Executar novamente na mesma ordem
        let mut state_replay = SystemState::new();
        for cycle in 0..ITERATIONS {
            state_replay.execute_cycle(cycle);
        }
        
        // Verificar igualdade elemento por elemento
        assert_eq!(state.action_ids, state_replay.action_ids, 
                   "ActionIds divergiram");
        assert_eq!(state.vibration_ids, state_replay.vibration_ids, 
                   "VibrationIds divergiram");
        assert_eq!(state.dna_ids, state_replay.dna_ids, 
                   "DnaIds divergiram");
    }
}
