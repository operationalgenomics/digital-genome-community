//! Canon v5.1 | GDC v1.0.0δ
//!
//! Swarm - Módulo de Teste de Enxame
//!
//! EXTERNO ao Core. Apenas teste/instrumentação.

pub mod orchestrator;

pub use orchestrator::{SwarmOrchestrator, GdcNode, SwarmMetrics, SwarmObservation};
