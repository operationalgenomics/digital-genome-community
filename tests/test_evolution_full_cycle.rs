use std::collections::HashMap;
use serde_json::json;
use digital_genome_community::core::types::ObservedAction;
use digital_genome_community::memory::provenance::FoucaultianTruth;
use digital_genome_community::engine::analysis::{AnalysisEngine, EvaluationModel};

#[test]
fn test_zone_integration_cycle() {
    println!("\n--- STARTING ZONE INTEGRATION (CORE -> MEMORY -> ENGINE) ---");

    // 1. ZONE 1 (CORE): Raw Evidence
    let context = HashMap::from([
        ("temperature".to_string(), 42.0),
        ("efficiency".to_string(), 0.95)
    ]);
    
    let action = ObservedAction::new(
        "sensor_alpha".to_string(),
        context,
        json!({"cmd": "cool_down"})
    );
    println!("[1] CORE: ObservedAction created. ID: {:?}", action.id);

    // 2. ZONE 2 (MEMORY): Crystallization (Foucault)
    let truth = FoucaultianTruth::new(action.clone(), "validator_node_01".to_string());
    println!("[2] MEMORY: FoucaultianTruth crystallized. Hash: {}", truth.integrity_hash);

    // 3. ZONE 3 (ENGINE): Evaluation
    // Use the default Praxeology model
    let model = EvaluationModel::default_praxeology();
    
    // Evaluate the Raw Fact embedded in the Truth
    let record = AnalysisEngine::evaluate_action(&truth.raw_fact, &model);

    println!("[3] ENGINE: Evaluation Record Generated.");
    println!("    Verdict: {}", record.verdict);
    println!("    Score: {:.4}", record.score_assigned);

    // Assertions
    assert_eq!(record.verdict, "APPROVED", "High context score should approve action.");
    assert!(record.score_assigned > 0.0, "Score must be positive.");

    println!("--- INTEGRATION SUCCESS ---\n");
}
