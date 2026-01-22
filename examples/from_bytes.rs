//! Example: Create input programmatically from bytes.
//!
//! This example demonstrates how to feed raw bytes to the system.
//! The bytes could come from any source: network, hardware, generation.
//! The system does not know or care about the origin.
//!
//! # Usage
//! ```bash
//! cargo run --example from_bytes
//! ```

use digital_genome_community::{RawInput, SensoryCortex};

fn main() {
    let cortex = SensoryCortex::new();
    
    println!("=== Epistemologically Neutral Perception ===");
    println!();
    println!("The system will perceive three different byte patterns.");
    println!("It does not know what these patterns represent.");
    println!();
    
    // Pattern 1: Constant bytes
    // The system will detect: low entropy, high compressibility
    // It does NOT know this is "constant" — it discovers the structure
    let constant_bytes: Vec<u8> = vec![42; 1000];
    demonstrate(&cortex, &constant_bytes, "Pattern A");
    
    // Pattern 2: Sequential bytes
    // The system will detect: moderate entropy, periodic structure
    // It does NOT know this is "sequential" — it discovers the structure
    let sequential_bytes: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
    demonstrate(&cortex, &sequential_bytes, "Pattern B");
    
    // Pattern 3: Mixed pattern
    // The system will detect: structure with variation
    // It does NOT know what this "is" — it discovers the structure
    let mixed_bytes: Vec<u8> = (0..1000)
        .map(|i| {
            if i % 100 < 50 {
                (i % 10) as u8
            } else {
                ((i * 7 + 3) % 256) as u8
            }
        })
        .collect();
    demonstrate(&cortex, &mixed_bytes, "Pattern C");
    
    println!("=== Summary ===");
    println!();
    println!("The system discovered structural differences between patterns");
    println!("without any knowledge of what the patterns represent.");
    println!();
    println!("Interpretation remains the responsibility of the human operator.");
}

fn demonstrate(cortex: &SensoryCortex, bytes: &[u8], label: &str) {
    let input = RawInput::from_bytes(bytes.to_vec());
    let output = cortex.perceive(&input);
    
    println!("--- {} ({} bytes) ---", label, bytes.len());
    println!("  Entropy:           {:.4}", output.signals.entropy);
    println!("  Compressibility:   {:.4}", output.signals.compressibility);
    println!("  Max autocorr:      {:.4}", output.signals.max_autocorrelation);
    println!("  Proto-agency:      {}", output.state_history.proto_agency_detected());
    println!();
}
