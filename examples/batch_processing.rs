//! Example: Process multiple inputs in batch.
//!
//! This example demonstrates batch processing of multiple files/inputs.
//! Each input is processed independently — the system maintains no state
//! between perceptions.
//!
//! # Usage
//! ```bash
//! cargo run --example batch_processing -- file1 file2 file3 ...
//! ```
//!
//! Or to process all files in a directory:
//! ```bash
//! cargo run --example batch_processing -- path/to/dir/*
//! ```

use digital_genome_community::{RawInput, SensoryCortex};
use std::env;
use std::fs;
use std::path::Path;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <file1> [file2] [file3] ...", args[0]);
        eprintln!();
        eprintln!("Process multiple files as raw bytes.");
        eprintln!("The system does not know what kind of files these are.");
        std::process::exit(1);
    }
    
    let file_paths: Vec<&String> = args.iter().skip(1).collect();
    
    println!("=== Batch Processing ===");
    println!();
    println!("Files to process: {}", file_paths.len());
    println!("Each file is treated as raw bytes — no format assumptions.");
    println!();
    
    // Single cortex instance — it's stateless, so safe to reuse
    let cortex = SensoryCortex::new();
    
    let mut results = Vec::new();
    let mut total_bytes = 0usize;
    let start = Instant::now();
    
    for path_str in &file_paths {
        let path = Path::new(path_str);
        
        // Skip directories
        if path.is_dir() {
            eprintln!("Skipping directory: {}", path_str);
            continue;
        }
        
        // Read as raw bytes
        let bytes = match fs::read(path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Error reading {}: {}", path_str, e);
                continue;
            }
        };
        
        total_bytes += bytes.len();
        
        // Create input and perceive
        let input = RawInput::from_bytes(bytes);
        let output = cortex.perceive(&input);
        
        results.push((path_str.to_string(), output));
    }
    
    let elapsed = start.elapsed();
    
    // Output results
    println!("=== Results ===");
    println!();
    
    for (path, output) in &results {
        let filename = Path::new(path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| path.clone());
            
        println!(
            "{:<30} entropy={:.3}  autocorr={:.3}  proto_agency={}",
            filename,
            output.signals.entropy,
            output.signals.max_autocorrelation,
            output.state_history.proto_agency_detected()
        );
    }
    
    println!();
    println!("=== Summary ===");
    println!("  Files processed: {}", results.len());
    println!("  Total bytes:     {}", total_bytes);
    println!("  Time elapsed:    {:?}", elapsed);
    println!("  Throughput:      {:.2} MB/s", 
        (total_bytes as f64 / 1_000_000.0) / elapsed.as_secs_f64());
    println!();
    println!("Note: Each file was perceived independently.");
    println!("      The system maintains no state between perceptions.");
}
