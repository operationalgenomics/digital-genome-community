//! MCI — Memória Cognitiva Interna — AF-12
//!
//! # Canon Reference
//! > "A MCI é um sistema de memória interna não-observation que armazena
//!    Códons Canônicos indexados por Contexto Canônico."
//!
//! # Key Properties
//! - NOT observation (does not store raw inputs)
//! - Participates in E3 pipeline stage (LEI-AF-12-04)
//! - Finite by non-dominance policy (LEI-AF-12-02)
//! - Serializable for replay (LEI-AF-12-05)
//!
//! # Layer: Community
//! # Version: 0.6.0 (MVP-6)

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::BTreeMap;

use super::context::CanonicalContext;
use super::codon::CanonicalCodon;

/// MCI Query Result
#[derive(Debug, Clone)]
pub struct MciQueryResult {
    /// Codons matching the query
    pub codons: Vec<CanonicalCodon>,
    /// Whether any results were found
    pub found: bool,
    /// Query context
    pub context: CanonicalContext,
}

/// MCI Statistics
#[derive(Debug, Clone, Copy, Default)]
pub struct MciStats {
    /// Total number of Codons
    pub total_codons: usize,
    /// Number of unique contexts
    pub unique_contexts: usize,
    /// Total incorporations
    pub incorporations: u64,
    /// Total discards by dominance
    pub discards_by_dominance: u64,
    /// Total rejections (vetoed)
    pub rejections_vetoed: u64,
}

/// Memória Cognitiva Interna — AF-12
///
/// Stores Canonical Codons indexed by Canonical Context.
///
/// # Canonical Compliance
/// - LEI-AF-12-01: Stores CanonicalCodon structures
/// - LEI-AF-12-02: Finite by non-dominance (CP comparison in same context)
/// - LEI-AF-12-04: Participates in pipeline (provides similar Codons)
/// - LEI-AF-12-05: Fully serializable for replay
///
/// # Thread Safety
/// This type is NOT internally synchronized. External synchronization
/// is required for concurrent access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCI {
    /// Codons indexed by context fingerprint
    codons: BTreeMap<[u8; 32], Vec<CanonicalCodon>>,
    
    /// Capacity limit (None = unlimited)
    capacity: Option<usize>,
    
    /// Statistics
    #[serde(skip)]
    stats: MciStats,
    
    /// Monotonic cycle counter for provenance
    cycle_counter: u64,
}

impl MCI {
    /// Create a new MCI with optional capacity limit.
    pub fn new(capacity: Option<usize>) -> Self {
        Self {
            codons: BTreeMap::new(),
            capacity,
            stats: MciStats::default(),
            cycle_counter: 0,
        }
    }
    
    /// Create an unlimited MCI.
    pub fn unlimited() -> Self {
        Self::new(None)
    }
    
    /// Create a limited MCI.
    pub fn with_capacity(max_codons: usize) -> Self {
        Self::new(Some(max_codons))
    }
    
    /// Get current cycle counter.
    pub fn cycle_counter(&self) -> u64 {
        self.cycle_counter
    }
    
    /// Increment and return cycle counter.
    pub fn next_cycle(&mut self) -> u64 {
        self.cycle_counter += 1;
        self.cycle_counter
    }
    
    /// Attempt to incorporate a Codon — LEI-AF-11-02
    ///
    /// # Incorporation Criteria
    /// 1. Codon must not be vetoed (LEI-AF-11-04)
    /// 2. CP_candidate > CP_baseline for same context (LEI-AF-11-02)
    ///
    /// # Returns
    /// - `Ok(true)` if incorporated
    /// - `Ok(false)` if not incorporated (dominated or vetoed)
    /// - `Err` if capacity exceeded and cannot evict
    pub fn try_incorporate(&mut self, codon: CanonicalCodon) -> Result<bool, MciError> {
        // LEI-AF-11-04: Vetoed Codons cannot be incorporated
        if !codon.can_incorporate() {
            self.stats.rejections_vetoed += 1;
            return Ok(false);
        }
        
        let context_key = codon.condicao_uso.context.fingerprint();
        
        // Check for dominance
        if let Some(existing) = self.codons.get(&context_key) {
            // Find best existing CP in this context
            let best_existing_cp = existing.iter()
                .map(|c| c.cp())
                .fold(0.0_f64, f64::max);
            
            // LEI-AF-11-02: Must improve strictly
            if codon.cp() <= best_existing_cp {
                return Ok(false);
            }
            
            // Remove dominated Codons (LEI-AF-12-02)
            let dominated_count = existing.iter()
                .filter(|c| codon.dominates(c))
                .count();
            self.stats.discards_by_dominance += dominated_count as u64;
        }
        
        // Check capacity
        if let Some(cap) = self.capacity {
            if self.total_codons() >= cap {
                // Try to evict lowest CP across all contexts
                if !self.evict_lowest() {
                    return Err(MciError::CapacityExceeded);
                }
            }
        }
        
        // Insert Codon
        let codons = self.codons.entry(context_key).or_insert_with(Vec::new);
        
        // Remove dominated Codons before inserting
        codons.retain(|c| !codon.dominates(c));
        codons.push(codon);
        
        self.stats.incorporations += 1;
        self.update_stats();
        
        Ok(true)
    }
    
    /// Query MCI for Codons similar to given context — LEI-AF-12-04
    ///
    /// Returns Codons that match the problem class.
    pub fn query(&self, context: &CanonicalContext) -> MciQueryResult {
        let context_key = context.fingerprint();
        
        // Exact match first
        if let Some(codons) = self.codons.get(&context_key) {
            return MciQueryResult {
                codons: codons.clone(),
                found: true,
                context: context.clone(),
            };
        }
        
        // Search for same problem class
        let similar: Vec<CanonicalCodon> = self.codons.values()
            .flat_map(|v| v.iter())
            .filter(|c| c.condicao_uso.context.same_problem_class(context))
            .cloned()
            .collect();
        
        MciQueryResult {
            found: !similar.is_empty(),
            codons: similar,
            context: context.clone(),
        }
    }
    
    /// Query MCI for best Codon in context (highest CP).
    pub fn query_best(&self, context: &CanonicalContext) -> Option<CanonicalCodon> {
        let result = self.query(context);
        result.codons.into_iter()
            .max_by(|a, b| a.cp().partial_cmp(&b.cp()).unwrap_or(std::cmp::Ordering::Equal))
    }
    
    /// Get baseline CP for context (best existing CP or 0).
    pub fn baseline_cp(&self, context: &CanonicalContext) -> f64 {
        self.query_best(context)
            .map(|c| c.cp())
            .unwrap_or(0.0)
    }
    
    /// Check if CP would improve over baseline — LEI-AF-11-02
    pub fn would_improve(&self, context: &CanonicalContext, candidate_cp: f64) -> bool {
        candidate_cp > self.baseline_cp(context)
    }
    
    /// Get total number of Codons.
    pub fn total_codons(&self) -> usize {
        self.codons.values().map(|v| v.len()).sum()
    }
    
    /// Get number of unique contexts.
    pub fn unique_contexts(&self) -> usize {
        self.codons.len()
    }
    
    /// Get statistics.
    pub fn stats(&self) -> MciStats {
        MciStats {
            total_codons: self.total_codons(),
            unique_contexts: self.unique_contexts(),
            ..self.stats
        }
    }
    
    /// Generate fingerprint of current MCI state — LEI-AF-12-05
    pub fn state_fingerprint(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.cycle_counter.to_le_bytes());
        hasher.update((self.total_codons() as u64).to_le_bytes());
        
        for (key, codons) in &self.codons {
            hasher.update(key);
            for codon in codons {
                hasher.update(&codon.fingerprint());
            }
        }
        
        hasher.finalize().into()
    }
    
    /// Clear all Codons (for testing/reset).
    pub fn clear(&mut self) {
        self.codons.clear();
        self.stats = MciStats::default();
    }
    
    /// Evict lowest CP Codon across all contexts.
    fn evict_lowest(&mut self) -> bool {
        let mut lowest_cp = f64::MAX;
        let mut lowest_key: Option<[u8; 32]> = None;
        let mut lowest_idx: Option<usize> = None;
        
        for (key, codons) in &self.codons {
            for (idx, codon) in codons.iter().enumerate() {
                if codon.cp() < lowest_cp {
                    lowest_cp = codon.cp();
                    lowest_key = Some(*key);
                    lowest_idx = Some(idx);
                }
            }
        }
        
        if let (Some(key), Some(idx)) = (lowest_key, lowest_idx) {
            if let Some(codons) = self.codons.get_mut(&key) {
                codons.remove(idx);
                if codons.is_empty() {
                    self.codons.remove(&key);
                }
                self.stats.discards_by_dominance += 1;
                return true;
            }
        }
        
        false
    }
    
    fn update_stats(&mut self) {
        self.stats.total_codons = self.total_codons();
        self.stats.unique_contexts = self.unique_contexts();
    }
}

impl Default for MCI {
    fn default() -> Self {
        Self::unlimited()
    }
}

/// MCI Error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MciError {
    /// Capacity exceeded and cannot evict
    CapacityExceeded,
    /// Invalid Codon
    InvalidCodon(String),
}

impl std::fmt::Display for MciError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MciError::CapacityExceeded => write!(f, "MCI capacity exceeded"),
            MciError::InvalidCodon(msg) => write!(f, "Invalid Codon: {}", msg),
        }
    }
}

impl std::error::Error for MciError {}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::codon::{EvaluativeSignature, ActivationCondition, ReplayableProvenance, Origin};
    
    fn make_codon(cp_root: f64, problem: &[u8], origin: Origin) -> CanonicalCodon {
        let ctx = CanonicalContext::new(problem, b"state");
        let sig = EvaluativeSignature::new(cp_root, cp_root, cp_root, cp_root, false);
        let mut prov = ReplayableProvenance::default();
        prov.origin = origin;
        
        CanonicalCodon::new(
            b"action".to_vec(),
            prov,
            sig,
            ActivationCondition::new(ctx),
        )
    }
    
    #[test]
    fn test_mci_creation() {
        let mci = MCI::unlimited();
        assert_eq!(mci.total_codons(), 0);
        assert_eq!(mci.unique_contexts(), 0);
    }
    
    #[test]
    fn test_incorporate_codon() {
        let mut mci = MCI::unlimited();
        let codon = make_codon(0.9, b"problem_a", Origin::External);
        
        let result = mci.try_incorporate(codon);
        assert!(result.is_ok());
        assert!(result.unwrap());
        assert_eq!(mci.total_codons(), 1);
    }
    
    #[test]
    fn test_vetoed_codon_rejected() {
        let mut mci = MCI::unlimited();
        let codon = make_codon(0.0, b"problem", Origin::External); // CP = 0^4 = 0
        
        let result = mci.try_incorporate(codon);
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Not incorporated
        assert_eq!(mci.total_codons(), 0);
    }
    
    #[test]
    fn test_dominance_policy() {
        let mut mci = MCI::unlimited();
        
        // Incorporate first Codon
        let codon1 = make_codon(0.8, b"problem_a", Origin::External);
        assert!(mci.try_incorporate(codon1).unwrap());
        assert_eq!(mci.total_codons(), 1);
        
        // Try to incorporate lower CP → rejected
        let codon2 = make_codon(0.5, b"problem_a", Origin::External);
        assert!(!mci.try_incorporate(codon2).unwrap());
        assert_eq!(mci.total_codons(), 1);
        
        // Incorporate higher CP → success, old is dominated
        let codon3 = make_codon(0.95, b"problem_a", Origin::External);
        assert!(mci.try_incorporate(codon3).unwrap());
        // Old Codon should be removed by dominance
        assert_eq!(mci.total_codons(), 1);
        
        // Verify the best CP is now 0.95^4
        let best = mci.query_best(&CanonicalContext::new(b"problem_a", b"state"));
        assert!(best.is_some());
        assert!((best.unwrap().cp() - 0.95_f64.powi(4)).abs() < 0.01);
    }
    
    #[test]
    fn test_query() {
        let mut mci = MCI::unlimited();
        
        let codon_a = make_codon(0.9, b"problem_a", Origin::External);
        let codon_b = make_codon(0.8, b"problem_b", Origin::External);
        
        mci.try_incorporate(codon_a).unwrap();
        mci.try_incorporate(codon_b).unwrap();
        
        let result = mci.query(&CanonicalContext::new(b"problem_a", b"state"));
        assert!(result.found);
        assert_eq!(result.codons.len(), 1);
    }
    
    #[test]
    fn test_baseline_cp() {
        let mut mci = MCI::unlimited();
        let ctx = CanonicalContext::new(b"problem", b"state");
        
        // No Codons → baseline = 0
        assert_eq!(mci.baseline_cp(&ctx), 0.0);
        
        // Add Codon
        let codon = make_codon(0.9, b"problem", Origin::External);
        mci.try_incorporate(codon).unwrap();
        
        // Baseline = best CP
        assert!((mci.baseline_cp(&ctx) - 0.9_f64.powi(4)).abs() < 0.01);
    }
    
    #[test]
    fn test_would_improve() {
        let mut mci = MCI::unlimited();
        let ctx = CanonicalContext::new(b"problem", b"state");
        
        // Empty MCI → any positive CP improves
        assert!(mci.would_improve(&ctx, 0.5));
        
        // Add Codon with CP = 0.9^4 ≈ 0.6561
        let codon = make_codon(0.9, b"problem", Origin::External);
        mci.try_incorporate(codon).unwrap();
        
        // Higher CP improves
        assert!(mci.would_improve(&ctx, 0.8));
        // Lower CP does not improve
        assert!(!mci.would_improve(&ctx, 0.5));
    }
    
    #[test]
    fn test_capacity_limit() {
        let mut mci = MCI::with_capacity(2);
        
        let codon1 = make_codon(0.9, b"problem_1", Origin::External);
        let codon2 = make_codon(0.8, b"problem_2", Origin::External);
        let codon3 = make_codon(0.7, b"problem_3", Origin::External);
        
        assert!(mci.try_incorporate(codon1).unwrap());
        assert!(mci.try_incorporate(codon2).unwrap());
        
        // Third should evict lowest (codon2 has 0.8^4 < 0.9^4)
        assert!(mci.try_incorporate(codon3).unwrap());
        assert!(mci.total_codons() <= 2);
    }
    
    #[test]
    fn test_state_fingerprint_determinism() {
        let mut mci1 = MCI::unlimited();
        let mut mci2 = MCI::unlimited();
        
        let codon = make_codon(0.9, b"problem", Origin::External);
        mci1.try_incorporate(codon.clone()).unwrap();
        mci2.try_incorporate(codon).unwrap();
        
        assert_eq!(mci1.state_fingerprint(), mci2.state_fingerprint());
    }
    
    #[test]
    fn test_cycle_counter() {
        let mut mci = MCI::unlimited();
        assert_eq!(mci.cycle_counter(), 0);
        
        assert_eq!(mci.next_cycle(), 1);
        assert_eq!(mci.next_cycle(), 2);
        assert_eq!(mci.cycle_counter(), 2);
    }
}
