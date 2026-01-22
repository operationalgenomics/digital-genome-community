//! Dataset Generator for Rigorous Validation
//!
//! This script generates synthetic datasets with known properties
//! for validating the Digital Genome Community Edition.
//!
//! Run with: cargo run --example generate_datasets

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    let output_dir = Path::new("validation/datasets");
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    println!("=== Generating Validation Datasets ===\n");

    // Dataset 1: Constant (zero entropy)
    generate_constant(output_dir);

    // Dataset 2: Ramp (sequential, growing entropy)
    generate_ramp(output_dir);

    // Dataset 3: Periodic (alternating pattern)
    generate_periodic(output_dir);

    // Dataset 4: Pseudo-random (high entropy)
    generate_pseudo_random(output_dir);

    // Dataset 5: Structured (local patterns)
    generate_structured(output_dir);

    // Dataset 6: Edge cases
    generate_edge_cases(output_dir);

    println!("\n=== All datasets generated successfully ===");
}

fn generate_constant(dir: &Path) {
    println!("Generating: constant.bin");
    
    // 1000 bytes of value 128 (middle value)
    let data: Vec<u8> = vec![128; 1000];
    write_dataset(dir, "constant.bin", &data);
    
    // Also generate constant at boundaries
    let data_zero: Vec<u8> = vec![0; 1000];
    write_dataset(dir, "constant_zero.bin", &data_zero);
    
    let data_max: Vec<u8> = vec![255; 1000];
    write_dataset(dir, "constant_max.bin", &data_max);
    
    println!("  - constant.bin: 1000 bytes of 128");
    println!("  - constant_zero.bin: 1000 bytes of 0");
    println!("  - constant_max.bin: 1000 bytes of 255");
}

fn generate_ramp(dir: &Path) {
    println!("Generating: ramp.bin");
    
    // Sequential values 0-255 repeated
    let data: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
    write_dataset(dir, "ramp.bin", &data);
    
    // Descending ramp
    let data_desc: Vec<u8> = (0..1000).map(|i| (255 - (i % 256)) as u8).collect();
    write_dataset(dir, "ramp_desc.bin", &data_desc);
    
    println!("  - ramp.bin: 0,1,2,...,255,0,1,2,...");
    println!("  - ramp_desc.bin: 255,254,...,0,255,254,...");
}

fn generate_periodic(dir: &Path) {
    println!("Generating: periodic.bin");
    
    // Binary alternation (0, 255, 0, 255, ...)
    let data_binary: Vec<u8> = (0..1000).map(|i| if i % 2 == 0 { 0 } else { 255 }).collect();
    write_dataset(dir, "periodic_binary.bin", &data_binary);
    
    // Period-4 pattern
    let pattern = [10, 50, 200, 100];
    let data_p4: Vec<u8> = (0..1000).map(|i| pattern[i % 4]).collect();
    write_dataset(dir, "periodic_p4.bin", &data_p4);
    
    // Period-10 pattern
    let pattern10 = [0, 25, 50, 75, 100, 125, 150, 175, 200, 225];
    let data_p10: Vec<u8> = (0..1000).map(|i| pattern10[i % 10]).collect();
    write_dataset(dir, "periodic_p10.bin", &data_p10);
    
    println!("  - periodic_binary.bin: 0,255,0,255,...");
    println!("  - periodic_p4.bin: period=4");
    println!("  - periodic_p10.bin: period=10");
}

fn generate_pseudo_random(dir: &Path) {
    println!("Generating: pseudo_random.bin");
    
    // LCG pseudo-random generator (deterministic)
    // Parameters: a=1103515245, c=12345, m=2^31
    let mut state: u64 = 42; // seed
    let data: Vec<u8> = (0..1000).map(|_| {
        state = (state.wrapping_mul(1103515245).wrapping_add(12345)) % (1u64 << 31);
        (state >> 16) as u8
    }).collect();
    write_dataset(dir, "pseudo_random_seed42.bin", &data);
    
    // Different seed
    let mut state2: u64 = 12345;
    let data2: Vec<u8> = (0..1000).map(|_| {
        state2 = (state2.wrapping_mul(1103515245).wrapping_add(12345)) % (1u64 << 31);
        (state2 >> 16) as u8
    }).collect();
    write_dataset(dir, "pseudo_random_seed12345.bin", &data2);
    
    println!("  - pseudo_random_seed42.bin: LCG with seed=42");
    println!("  - pseudo_random_seed12345.bin: LCG with seed=12345");
}

fn generate_structured(dir: &Path) {
    println!("Generating: structured.bin");
    
    // Local structure: blocks of similar values
    let mut data = Vec::with_capacity(1000);
    for block in 0..10 {
        let base = (block * 25) as u8;
        for _ in 0..100 {
            // Small variation within block
            let variation = ((block * 7) % 5) as u8;
            data.push(base.wrapping_add(variation));
        }
    }
    write_dataset(dir, "structured_blocks.bin", &data);
    
    // Gradient with noise
    let data_gradient: Vec<u8> = (0..1000).map(|i| {
        let base = ((i as f64 / 1000.0) * 255.0) as u8;
        let noise = ((i * 7) % 10) as u8;
        base.wrapping_add(noise)
    }).collect();
    write_dataset(dir, "structured_gradient.bin", &data_gradient);
    
    println!("  - structured_blocks.bin: 10 blocks of similar values");
    println!("  - structured_gradient.bin: gradient with noise");
}

fn generate_edge_cases(dir: &Path) {
    println!("Generating: edge_cases");
    
    // Empty (will be handled separately in tests)
    write_dataset(dir, "empty.bin", &[]);
    
    // Single byte
    write_dataset(dir, "single_byte.bin", &[42]);
    
    // Two bytes (minimum for some analyses)
    write_dataset(dir, "two_bytes.bin", &[0, 255]);
    
    // Very short periodic
    write_dataset(dir, "short_periodic.bin", &[1, 2, 1, 2, 1, 2, 1, 2]);
    
    // All unique values (maximum entropy for size)
    let unique: Vec<u8> = (0..=255).collect();
    write_dataset(dir, "all_unique.bin", &unique);
    
    // Permutation pair (same mean, different order)
    write_dataset(dir, "permutation_a.bin", &[1, 2, 3, 4, 5]);
    write_dataset(dir, "permutation_b.bin", &[5, 4, 3, 2, 1]);
    
    println!("  - empty.bin: 0 bytes");
    println!("  - single_byte.bin: 1 byte");
    println!("  - two_bytes.bin: 2 bytes");
    println!("  - short_periodic.bin: 8 bytes periodic");
    println!("  - all_unique.bin: 256 unique values");
    println!("  - permutation_a/b.bin: same mean, different order");
}

fn write_dataset(dir: &Path, name: &str, data: &[u8]) {
    let path = dir.join(name);
    let mut file = File::create(&path).expect("Failed to create file");
    file.write_all(data).expect("Failed to write data");
}
