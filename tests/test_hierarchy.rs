use digital_genome_community::topology::network::{CognitiveNetwork, OntogenesisState};
use digital_genome_community::topology::connections::Synapse;
use digital_genome_community::core::identifiers::DnaId;
use uuid::Uuid;

#[test]
fn test_topology_lifecycle() {
    println!("\n--- STARTING TOPOLOGY SIMULATION (ZONE 4) ---");

    // 1. BOOTSTRAP: Create the Network (Brain)
    let mut network = CognitiveNetwork::new("Unit_Test_Net_01".to_string());
    
    // ASSERT: System must start in Fetus state
    network.check_vital_signs();
    assert_eq!(network.state, OntogenesisState::Fetus, "Network must be born as Fetus.");
    println!("[1] Network Born. Identity: {}", network.identity);

    // 2. STRUCTURE: Create a Neuron
    let neuron = network.grow_neuron("maintenance.welding".to_string());
    
    // 3. LEARNING: Attach a Synapse
    // We mock a DNA ID since the synapse points to an abstract ideal
    let target_dna = DnaId(Uuid::new_v4());
    let mut synapse = Synapse::new("arc_stability".to_string(), target_dna);
    
    // Reinforce learning (Plasticity)
    synapse.reinforce(0.5);
    neuron.attach_synapse(synapse);

    println!("[2] Neuron Grown. Activation Level: {:.2}", neuron.activation_level);

    // 4. ONTOGENESIS: Check Viability
    network.check_vital_signs();
    
    println!("[3] Vital Signs Checked. State: {:?}", network.state);
    
    // With 1 neuron and plasticity > 0.1, it should be Viable
    assert_eq!(network.state, OntogenesisState::Viable, "Network failed to mature.");

    println!("--- TOPOLOGY SUCCESS ---\n");
}
