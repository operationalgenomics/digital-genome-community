//! Example: Load any file as bytes and perceive it.
//!
//! This example demonstrates the epistemologically neutral approach:
//! - The system does not know what kind of file it is processing
//! - All files are treated as raw bytes
//! - No domain-specific preprocessing is applied
//!
//! # Usage
//! ```bash
//! cargo run --example from_file -- path/to/any/file
//! ```

use digital_genome_community::{RawInput, SensoryCortex};
use std::env;
use std::fs;
use std::process;

fn main() {
    // Get file path from command line
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        eprintln!();
        eprintln!("The system will perceive the file as raw bytes.");
        eprintln!("It does not know or care what kind of file it is.");
        process::exit(1);
    }
    
    let file_path = &args[1];
    
    // Read file as raw bytes — no parsing, no interpretation
    let bytes = match fs::read(file_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };
    
    println!("File: {}", file_path);
    println!("Size: {} bytes", bytes.len());
    println!();
    
    // Create input — just bytes, nothing more
    let input = RawInput::from_bytes(bytes);
    
    // Perceive — the system discovers structure without knowing what it's looking at
    let cortex = SensoryCortex::new();
    let output = cortex.perceive(&input);
    
    // Output results
    println!("=== Perception Results ===");
    println!();
    println!("Signals:");
    println!("  Entropy (normalized):     {:.4}", output.signals.entropy);
    println!("  Max autocorrelation:      {:.4}", output.signals.max_autocorrelation);
    println!("  Spectral flatness:        {:.4}", output.signals.spectral_flatness);
    println!("  Local/global entropy:     {:.4}", output.signals.local_global_entropy_ratio);
    println!("  Compressibility:          {:.4}", output.signals.compressibility);
    println!();
    println!("State:");
    println!("  Final state:              {}", output.state_history.current().name());
    println!("  Proto-agency detected:    {}", output.state_history.proto_agency_detected());
    println!("  Transitions:              {}", output.state_history.transitions().len());
    println!();
    println!("Note: The system perceived this data without knowing what it represents.");
    println!("      Interpretation is the responsibility of the human operator.");
}
