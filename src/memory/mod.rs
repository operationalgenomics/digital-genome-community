//! Memory Module — AF-11, AF-12, AO-18
//!
//! # Canon References
//! - AF-11: Aprendizado Cognitivo Autônomo por Incorporação Replayável
//! - AF-12: Memória Cognitiva Interna (MCI) — Não-Observation
//! - AO-18: Autorreferência Cognitiva Operacional (Origin marker)
//!
//! # Overview
//!
//! This module implements the cognitive memory system for GDC:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                     MEMORY SYSTEM                           │
//! ├─────────────────────────────────────────────────────────────┤
//! │                                                             │
//! │  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
//! │  │  Context    │───→│   Codon     │───→│    MCI      │     │
//! │  │  (where)    │    │   (what)    │    │   (store)   │     │
//! │  └─────────────┘    └─────────────┘    └─────────────┘     │
//! │                            │                  ▲             │
//! │                            ▼                  │             │
//! │                     ┌─────────────┐           │             │
//! │                     │  Learning   │───────────┘             │
//! │                     │  Engine     │                         │
//! │                     └─────────────┘                         │
//! │                                                             │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Components
//!
//! - **CanonicalContext**: Problem class + initial conditions (LEI-AF-12-02)
//! - **CanonicalCodon**: Atomic knowledge unit with 4 fields (LEI-AF-12-01)
//! - **MCI**: Memory store indexed by context (AF-12)
//! - **LearningEngine**: Incorporation orchestrator (AF-11)
//! - **Origin**: Autorreferência marker (AO-18)
//!
//! # Layer: Community
//! # Version: 0.6.0 (MVP-6)

mod context;
mod codon;
mod mci;
mod learning;

// Re-exports
pub use context::CanonicalContext;
pub use codon::{
    CanonicalCodon,
    Origin,
    EvaluativeSignature,
    ActivationCondition,
    ReplayableProvenance,
};
pub use mci::{MCI, MciQueryResult, MciStats, MciError};
pub use learning::{
    LearningEngine,
    LearningResult,
    EpistemicTrigger,
    RejectionReason,
    StagnationDetector,
};

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Integration test: Complete learning cycle
    #[test]
    fn test_complete_learning_cycle() {
        // 1. Create MCI
        let mut mci = MCI::unlimited();
        
        // 2. Create Learning Engine
        let mut engine = LearningEngine::without_replay_verify(2);
        
        // 3. Create context
        let ctx = CanonicalContext::new(b"turbine_analysis", b"initial_state");
        
        // 4. Simulate cognitive cycle with low CP
        let sig_low = EvaluativeSignature::new(0.7, 0.7, 0.7, 0.7, false);
        let codon_low = engine.create_candidate(
            b"analyze_turbine_v1".to_vec(),
            ctx.clone(),
            sig_low,
            [1; 16],
            [2; 32],
            [0; 32],
            1,
            Origin::External,
        );
        
        // 5. Learn first Codon
        let result1 = engine.try_learn(&mut mci, codon_low, EpistemicTrigger::ExplicitRequest);
        assert!(result1.learned);
        assert_eq!(mci.total_codons(), 1);
        
        // 6. Check for stagnation (same CP)
        let baseline = mci.baseline_cp(&ctx);
        assert!(engine.check_trigger(&ctx, baseline).is_none());
        assert!(engine.check_trigger(&ctx, baseline).is_none());
        // Third same CP → trigger
        let trigger = engine.check_trigger(&ctx, baseline);
        assert!(matches!(trigger, Some(EpistemicTrigger::Stagnation)));
        
        // 7. Meristic proposes improvement
        let sig_high = EvaluativeSignature::new(0.9, 0.9, 0.9, 0.9, false);
        let codon_high = engine.create_candidate(
            b"analyze_turbine_v2".to_vec(),
            ctx.clone(),
            sig_high,
            [2; 16],
            [3; 32],
            mci.state_fingerprint(),
            2,
            Origin::Internal, // From Meristic
        );
        
        // 8. Learn improved Codon
        let result2 = engine.try_learn(&mut mci, codon_high, EpistemicTrigger::MeristicProposal);
        assert!(result2.learned);
        assert!(result2.new_cp.unwrap() > result2.baseline_cp);
        
        // 9. Old Codon dominated and removed
        assert_eq!(mci.total_codons(), 1);
        
        // 10. Best Codon has high CP
        let best = mci.query_best(&ctx).unwrap();
        assert!(best.cp() > 0.6); // 0.9^4 = 0.6561
        assert_eq!(best.origin(), Origin::Internal);
    }
    
    /// Test Origin marker consistency (AO-18)
    #[test]
    fn test_origin_marker_consistency() {
        let mut mci = MCI::unlimited();
        let mut engine = LearningEngine::without_replay_verify(3);
        
        let ctx = CanonicalContext::new(b"problem", b"state");
        
        // External Codon
        let sig = EvaluativeSignature::new(0.8, 0.8, 0.8, 0.8, false);
        let codon_ext = engine.create_candidate(
            b"action_ext".to_vec(),
            ctx.clone(),
            sig,
            [1; 16], [0; 32], [0; 32], 1,
            Origin::External,
        );
        engine.try_learn(&mut mci, codon_ext, EpistemicTrigger::ExplicitRequest);
        
        // Query and verify origin
        let result = mci.query(&ctx);
        assert!(result.found);
        assert_eq!(result.codons[0].origin(), Origin::External);
        
        // Internal Codon (better)
        let sig2 = EvaluativeSignature::new(0.95, 0.95, 0.95, 0.95, false);
        let codon_int = engine.create_candidate(
            b"action_int".to_vec(),
            ctx.clone(),
            sig2,
            [2; 16], [0; 32], [0; 32], 2,
            Origin::Internal,
        );
        engine.try_learn(&mut mci, codon_int, EpistemicTrigger::MeristicProposal);
        
        // Best is now Internal
        let best = mci.query_best(&ctx).unwrap();
        assert_eq!(best.origin(), Origin::Internal);
    }
    
    /// Test thread safety markers
    #[test]
    fn test_types_are_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        
        assert_send_sync::<CanonicalContext>();
        assert_send_sync::<CanonicalCodon>();
        assert_send_sync::<Origin>();
        assert_send_sync::<EvaluativeSignature>();
        assert_send_sync::<MCI>();
        // LearningEngine uses HashMap which is Send+Sync
    }
}
