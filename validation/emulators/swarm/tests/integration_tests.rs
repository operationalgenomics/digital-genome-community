//! Canon v5.1 | GDC v1.0.0δ
//!
//! Testes de Integração - Enxame N ≥ 10

use gdc_emulators::swarm::{SwarmOrchestrator, GdcNode};
use digital_genome_community::coordination::manifestation_delta::GdcId;

#[test]
fn test_swarm_with_10_gdcs() {
    let mut orchestrator = SwarmOrchestrator::new();
    
    // Criar 10 GDCs
    for _ in 0..10 {
        let gdc = GdcNode::new(GdcId::new());
        orchestrator.add_gdc(gdc);
    }
    
    // Observar ciclo
    let stimulus = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let observation = orchestrator.observe_swarm_cycle(stimulus);
    
    // Validações
    assert!(observation.success, "Cycle should succeed");
    assert!(observation.queen_id.is_some(), "Should have Queen");
    assert!(observation.worker_count >= 1, "Should have Workers");
    
    // Métricas
    let metrics = orchestrator.metrics();
    assert_eq!(metrics.vibrations_emitted, 1);
    assert!(metrics.manifestations_received >= 1);
}

#[test]
fn test_swarm_fails_with_less_than_10() {
    let mut orchestrator = SwarmOrchestrator::new();
    
    // Apenas 5 GDCs
    for _ in 0..5 {
        orchestrator.add_gdc(GdcNode::new(GdcId::new()));
    }
    
    let observation = orchestrator.observe_swarm_cycle(vec![1, 2, 3]);
    
    assert!(!observation.success);
    assert!(observation.error.is_some());
}

#[test]
fn test_swarm_with_15_gdcs() {
    let mut orchestrator = SwarmOrchestrator::new();
    
    // 15 GDCs
    for _ in 0..15 {
        orchestrator.add_gdc(GdcNode::new(GdcId::new()));
    }
    
    let observation = orchestrator.observe_swarm_cycle(vec![1; 100]);
    
    assert!(observation.success);
    assert!(observation.worker_count >= 10);
}

#[test]
fn test_multiple_cycles() {
    let mut orchestrator = SwarmOrchestrator::new();
    
    for _ in 0..10 {
        orchestrator.add_gdc(GdcNode::new(GdcId::new()));
    }
    
    // Múltiplos ciclos
    for i in 0..5 {
        let stimulus = vec![i; 10];
        let obs = orchestrator.observe_swarm_cycle(stimulus);
        assert!(obs.success, "Cycle {} should succeed", i);
    }
    
    let metrics = orchestrator.metrics();
    assert_eq!(metrics.vibrations_emitted, 5);
}
