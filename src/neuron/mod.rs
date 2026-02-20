//! Canon v5.1 | GDC v0.9.5
//!
//! Neuron module - Agrupamentos funcionais emergentes
//!
//! Canon: AF-15 (Ressonância Estrutural)

pub mod activation;
pub mod cluster;

pub use activation::{ActivationPattern, ActivationTracker};
pub use cluster::{NeuronCluster, ClusterNetwork};
