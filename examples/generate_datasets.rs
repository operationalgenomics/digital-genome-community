//! Dataset Generator for Rigorous Validation
//!
//! Generates synthetic datasets with known properties for validation.
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

    // Dataset 2: Ramp (sequential)
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
    println!("Location: validation/datasets/");
}

fn generate_constant(dir: &Path) {
    println!("Generating: constant datasets");
    
    let data: Vec<u8> = vec![128; 1000];
    write_dataset(dir, "constant.bin", &data);
    
    let data_zero: Vec<u8> = vec![0; 1000];
    write_dataset(dir, "constant_zero.bin", &data_zero);
    
    let data_max: Vec<u8> = vec![255; 1000];
    write_dataset(dir, "constant_max.bin", &data_max);
    
    println!("  - constant.bin: 1000 bytes of 128");
    println!("  - constant_zero.bin: 1000 bytes of 0");
    println!("  - constant_max.bin: 1000 bytes of 255");
}

fn generate_ramp(dir: &Path) {
    println!("Generating: ramp datasets");
    
    let data: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
    write_dataset(dir, "ramp.bin", &data);
    
    let data_desc: Vec<u8> = (0..1000).map(|i| (255 - (i % 256)) as u8).collect();
    write_dataset(dir, "ramp_desc.bin", &data_desc);
    
    println!("  - ramp.bin: ascending 0,1,2,...,255,...");
    println!("  - ramp_desc.bin: descending 255,254,...,0,...");
}

fn generate_periodic(dir: &Path) {
    println!("Generating: periodic datasets");
    
    let data_binary: Vec<u8> = (0..1000).map(|i| if i % 2 == 0 { 0 } else { 255 }).collect();
    write_dataset(dir, "periodic_binary.bin", &data_binary);
    
    let pattern = [10u8, 50, 200, 100];
    let data_p4: Vec<u8> = (0..1000).map(|i| pattern[i % 4]).collect();
    write_dataset(dir, "periodic_p4.bin", &data_p4);
    
    let pattern10 = [0u8, 25, 50, 75, 100, 125, 150, 175, 200, 225];
    let data_p10: Vec<u8> = (0..1000).map(|i| pattern10[i % 10]).collect();
    write_dataset(dir, "periodic_p10.bin", &data_p10);
    
    println!("  - periodic_binary.bin: 0,255,0,255,...");
    println!("  - periodic_p4.bin: period=4");
    println!("  - periodic_p10.bin: period=10");
}

fn generate_pseudo_random(dir: &Path) {
    println!("Generating: pseudo-random datasets");
    
    let mut state: u64 = 42;
    let data: Vec<u8> = (0..1000).map(|_| {
        state = state.wrapping_mul(1103515245).wrapping_add(12345) % (1u64 << 31);
        (state >> 16) as u8
    }).collect();
    write_dataset(dir, "pseudo_random_seed42.bin", &data);
    
    let mut state2: u64 = 12345;
    let data2: Vec<u8> = (0..1000).map(|_| {
        state2 = state2.wrapping_mul(1103515245).wrapping_add(12345) % (1u64 << 31);
        (state2 >> 16) as u8
    }).collect();
    write_dataset(dir, "pseudo_random_seed12345.bin", &data2);
    
    println!("  - pseudo_random_seed42.bin: LCG seed=42");
    println!("  - pseudo_random_seed12345.bin: LCG seed=12345");
}

fn generate_structured(dir: &Path) {
    println!("Generating: structured datasets");
    
    let mut data = Vec::with_capacity(1000);
    for block in 0..10 {
        let base = (block * 25) as u8;
        for _ in 0..100 {
            let variation = ((block * 7) % 5) as u8;
            data.push(base.wrapping_add(variation));
        }
    }
    write_dataset(dir, "structured_blocks.bin", &data);
    
    let data_gradient: Vec<u8> = (0..1000).map(|i| {
        let base = ((i as f64 / 1000.0) * 255.0) as u8;
        let noise = ((i * 7) % 10) as u8;
        base.wrapping_add(noise)
    }).collect();
    write_dataset(dir, "structured_gradient.bin", &data_gradient);
    
    println!("  - structured_blocks.bin: 10 blocks");
    println!("  - structured_gradient.bin: gradient with noise");
}

fn generate_edge_cases(dir: &Path) {
    println!("Generating: edge case datasets");
    
    write_dataset(dir, "empty.bin", &[]);
    write_dataset(dir, "single_byte.bin", &[42]);
    write_dataset(dir, "two_bytes.bin", &[0, 255]);
    write_dataset(dir, "short_periodic.bin", &[1, 2, 1, 2, 1, 2, 1, 2]);
    
    let unique: Vec<u8> = (0..=255).collect();
    write_dataset(dir, "all_unique.bin", &unique);
    
    write_dataset(dir, "permutation_a.bin", &[1, 2, 3, 4, 5]);
    write_dataset(dir, "permutation_b.bin", &[5, 4, 3, 2, 1]);
    
    println!("  - empty.bin, single_byte.bin, two_bytes.bin");
    println!("  - short_periodic.bin, all_unique.bin");
    println!("  - permutation_a.bin, permutation_b.bin");
}

fn write_dataset(dir: &Path, name: &str, data: &[u8]) {
    let path = dir.join(name);
    let mut file = File::create(&path).expect("Failed to create file");
    file.write_all(data).expect("Failed to write data");
}
