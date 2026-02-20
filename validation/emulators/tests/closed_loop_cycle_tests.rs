//! Canon v5.1 | GDC v1.0.0γ
//!
//! Closed-Loop Cycle Tests - Teste de Ciclo Fechado com Persistência Real

use gdc_emulators::orchestrator::CycleOrchestrator;
use std::env;

#[test]
fn test_1000_cycles_with_real_persistence() {
    // v1.0.0γ: Ciclo fechado com persistência real
    let temp_dir = env::temp_dir().join(format!("gamma_1000_{}", rand::random::<u64>()));
    
    println!("\n=== TESTE v1.0.0γ: 1000 CICLOS COM PERSISTÊNCIA REAL ===");
    println!("Storage: {:?}", temp_dir);
    
    let mut orchestrator = CycleOrchestrator::new(&temp_dir)
        .expect("Failed to create orchestrator");
    
    // Executar 1000 ciclos
    let identity_preserved = orchestrator.run_cycles(1000)
        .expect("Failed to run cycles");
    
    // Verificações
    assert!(identity_preserved, "Identity must be preserved across all cycles");
    
    let metrics = orchestrator.metrics();
    assert_eq!(metrics.total_cycles, 1000);
    assert_eq!(metrics.successful_cycles, 1000);
    assert_eq!(metrics.failed_cycles, 0);
    assert_eq!(metrics.identity_breaks, 0);
    
    // Verificar storage
    assert_eq!(orchestrator.storage().count(), 1000);
    
    // Verificar lineage
    let lineage = orchestrator.get_lineage().unwrap();
    assert_eq!(lineage.len(), 1000);
    assert_eq!(lineage[0], "dna_000001");
    assert_eq!(lineage[999], "dna_001000");
    
    println!("\n✅ RESULTADO:");
    println!("  Total de ciclos: {}", metrics.total_cycles);
    println!("  Ciclos bem-sucedidos: {}", metrics.successful_cycles);
    println!("  Quebras de identidade: {}", metrics.identity_breaks);
    println!("  Duração total: {:?}", metrics.total_duration);
    println!("  Média por ciclo: {:?}", metrics.avg_cycle_duration);
    println!("  DNAs persistidos: {}", orchestrator.storage().count());
    println!("  Lineage completa: {} gerações", lineage.len());
    
    // Cleanup
    std::fs::remove_dir_all(temp_dir).ok();
}

#[test]
fn test_cycle_checkpoint_every_100() {
    // Validar que checkpoints funcionam
    let temp_dir = env::temp_dir().join(format!("gamma_checkpoint_{}", rand::random::<u64>()));
    
    let mut orchestrator = CycleOrchestrator::new(&temp_dir).unwrap();
    
    // 300 ciclos deve gerar 3 checkpoints (100, 200, 300)
    orchestrator.run_cycles(300).unwrap();
    
    assert_eq!(orchestrator.storage().count(), 300);
    
    // Verificar DNAs específicos existem
    let dna_100 = orchestrator.storage().recall_by_generation(100).unwrap();
    let dna_200 = orchestrator.storage().recall_by_generation(200).unwrap();
    let dna_300 = orchestrator.storage().recall_by_generation(300).unwrap();
    
    assert_eq!(dna_100.generation, 100);
    assert_eq!(dna_200.generation, 200);
    assert_eq!(dna_300.generation, 300);
    
    std::fs::remove_dir_all(temp_dir).ok();
}

#[test]
fn test_cycle_persistence_survives_restart() {
    // Ciclo deve sobreviver a reinício do orchestrator
    let temp_dir = env::temp_dir().join(format!("gamma_restart_{}", rand::random::<u64>()));
    
    // Primeira execução: 50 ciclos
    {
        let mut orchestrator = CycleOrchestrator::new(&temp_dir).unwrap();
        orchestrator.run_cycles(50).unwrap();
        assert_eq!(orchestrator.storage().count(), 50);
    }
    
    // Segunda execução: carregar storage existente
    {
        let orchestrator = CycleOrchestrator::new(&temp_dir).unwrap();
        
        // Deve carregar 50 DNAs existentes
        assert_eq!(orchestrator.storage().count(), 50);
        
        // Deve conseguir recuperar DNAs
        let dna_1 = orchestrator.storage().recall("dna_000001").unwrap();
        let dna_50 = orchestrator.storage().recall("dna_000050").unwrap();
        
        assert_eq!(dna_1.generation, 1);
        assert_eq!(dna_50.generation, 50);
    }
    
    std::fs::remove_dir_all(temp_dir).ok();
}

#[test]
fn test_1000_cycles_persistent_validation() {
    // v1.0.0γ VALIDAÇÃO: Mantém storage para inspeção manual
    // Este teste NÃO deleta o diretório de storage
    
    let storage_path = std::env::current_dir()
        .unwrap()
        .join("test_storage_gamma_1000");
    
    // Limpar se existir de execução anterior
    std::fs::remove_dir_all(&storage_path).ok();
    
    println!("\n=== VALIDAÇÃO MANUAL v1.0.0γ: 1000 CICLOS ===");
    println!("Storage permanente: {:?}", storage_path);
    println!("ATENÇÃO: Diretório NÃO será deletado após teste");
    println!("Você pode inspecionar manualmente os arquivos.");
    
    let mut orchestrator = CycleOrchestrator::new(&storage_path)
        .expect("Failed to create orchestrator");
    
    // Executar 1000 ciclos
    let identity_preserved = orchestrator.run_cycles(1000)
        .expect("Failed to run cycles");
    
    assert!(identity_preserved);
    
    let metrics = orchestrator.metrics();
    assert_eq!(metrics.total_cycles, 1000);
    assert_eq!(metrics.identity_breaks, 0);
    
    // Verificar arquivos no disco
    let dna_files: Vec<_> = std::fs::read_dir(&storage_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
        .collect();
    
    println!("\n✅ ARQUIVOS PERSISTIDOS:");
    println!("  Diretório: {:?}", storage_path);
    println!("  Total de arquivos JSON: {}", dna_files.len());
    println!("  Esperado: 1001 (1000 DNAs + 1 index)");
    
    assert!(dna_files.len() >= 1000, "Deveria ter ao menos 1000 DNAs persistidos");
    
    println!("\n✅ VALIDAÇÃO:");
    println!("  Para inspecionar:");
    println!("    cd validation/emulators");
    println!("    ls -la test_storage_gamma_1000/");
    println!("    cat test_storage_gamma_1000/dna_000001.json");
    println!("    cat test_storage_gamma_1000/index.json");
    
    // NÃO deletar: std::fs::remove_dir_all(storage_path).ok();
}

#[test]
fn test_cycle_dna_integrity() {
    // Validar que todos DNAs mantêm integridade
    let temp_dir = env::temp_dir().join(format!("gamma_integrity_{}", rand::random::<u64>()));
    
    let mut orchestrator = CycleOrchestrator::new(&temp_dir).unwrap();
    orchestrator.run_cycles(100).unwrap();
    
    // Verificar integridade de todos DNAs
    for id in orchestrator.storage().list_ids() {
        let dna = orchestrator.storage().recall(&id).unwrap();
        assert!(dna.verify(), "DNA {} failed integrity check", id);
    }
    
    std::fs::remove_dir_all(temp_dir).ok();
}
