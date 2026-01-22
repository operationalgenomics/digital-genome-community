//! Example: Multithread processing demonstration.
//!
//! This example demonstrates that:
//! 1. SensoryCortex is thread-safe (Send + Sync)
//! 2. Multiple threads can perceive simultaneously
//! 3. Each perception is isolated — no shared state
//! 4. Results are deterministic — same input → same output
//!
//! # Usage
//! ```bash
//! cargo run --example multithread_demo
//! cargo run --example multithread_demo -- 8   # Use 8 threads
//! ```
//!
//! # Threading Model
//! - Community Edition: provides thread-safe primitives
//! - Enterprise Edition: orchestrates threads (not shown here)
//! - This example shows raw thread-safety, not orchestration

use digital_genome_community::{RawInput, SensoryCortex};
use std::env;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_threads: usize = args.get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(4);
    
    let inputs_per_thread = 100;
    let total_inputs = num_threads * inputs_per_thread;
    
    println!("=== Multithread Demonstration ===");
    println!();
    println!("Threads:           {}", num_threads);
    println!("Inputs per thread: {}", inputs_per_thread);
    println!("Total inputs:      {}", total_inputs);
    println!();
    
    // Create shared cortex — it's stateless, so this is safe
    // Arc allows sharing across threads (Send + Sync verified at compile time)
    let cortex = Arc::new(SensoryCortex::new());
    
    // Generate test inputs — just bytes, no domain knowledge
    // Each thread will process different inputs
    let all_inputs: Vec<Vec<u8>> = (0..total_inputs)
        .map(|i| generate_deterministic_bytes(i, 1000))
        .collect();
    
    println!("=== Phase 1: Parallel Processing ===");
    println!();
    
    let start = Instant::now();
    
    // Spawn threads
    let handles: Vec<_> = (0..num_threads)
        .map(|thread_id| {
            let cortex = Arc::clone(&cortex);
            let inputs: Vec<Vec<u8>> = all_inputs
                .iter()
                .skip(thread_id * inputs_per_thread)
                .take(inputs_per_thread)
                .cloned()
                .collect();
            
            thread::spawn(move || {
                let mut results = Vec::with_capacity(inputs_per_thread);
                
                for bytes in inputs {
                    let input = RawInput::from_bytes(bytes);
                    let output = cortex.perceive(&input);
                    results.push(output.signals.entropy);
                }
                
                (thread_id, results)
            })
        })
        .collect();
    
    // Collect results
    let mut all_results: Vec<(usize, Vec<f64>)> = handles
        .into_iter()
        .map(|h| h.join().expect("Thread panicked"))
        .collect();
    
    all_results.sort_by_key(|(id, _)| *id);
    
    let parallel_elapsed = start.elapsed();
    
    // Flatten results for verification
    let parallel_entropies: Vec<f64> = all_results
        .iter()
        .flat_map(|(_, results)| results.clone())
        .collect();
    
    println!("Parallel processing complete.");
    println!("Time: {:?}", parallel_elapsed);
    println!();
    
    // Phase 2: Verify determinism with sequential processing
    println!("=== Phase 2: Determinism Verification ===");
    println!();
    
    let start = Instant::now();
    
    let sequential_entropies: Vec<f64> = all_inputs
        .iter()
        .map(|bytes| {
            let input = RawInput::from_bytes(bytes.clone());
            let output = cortex.perceive(&input);
            output.signals.entropy
        })
        .collect();
    
    let sequential_elapsed = start.elapsed();
    
    println!("Sequential processing complete.");
    println!("Time: {:?}", sequential_elapsed);
    println!();
    
    // Verify determinism
    println!("=== Phase 3: Determinism Check ===");
    println!();
    
    let mut mismatches = 0;
    for (i, (par, seq)) in parallel_entropies.iter().zip(&sequential_entropies).enumerate() {
        if (par - seq).abs() > f64::EPSILON {
            println!("MISMATCH at {}: parallel={} sequential={}", i, par, seq);
            mismatches += 1;
        }
    }
    
    if mismatches == 0 {
        println!("✓ All {} results match exactly.", total_inputs);
        println!("✓ Parallel processing is deterministic.");
    } else {
        println!("✗ {} mismatches detected!", mismatches);
    }
    
    println!();
    println!("=== Summary ===");
    println!();
    println!("Parallel time:    {:?}", parallel_elapsed);
    println!("Sequential time:  {:?}", sequential_elapsed);
    println!("Speedup:          {:.2}x", 
        sequential_elapsed.as_secs_f64() / parallel_elapsed.as_secs_f64());
    println!();
    println!("Threading Model:");
    println!("  - Each thread called perceive() independently");
    println!("  - No locks, no shared state, no synchronization");
    println!("  - Community Edition provides thread-safety");
    println!("  - Enterprise Edition would orchestrate (not shown)");
}

/// Generates deterministic bytes from a seed.
/// This is NOT random — same seed always produces same output.
/// This allows verification of determinism across threads.
fn generate_deterministic_bytes(seed: usize, length: usize) -> Vec<u8> {
    // Simple deterministic generator — NOT cryptographic, just for demo
    // Uses seed to create reproducible byte patterns
    (0..length)
        .map(|i| {
            let x = seed.wrapping_mul(31).wrapping_add(i);
            let y = x.wrapping_mul(17).wrapping_add(seed);
            ((x ^ y) % 256) as u8
        })
        .collect()
}
