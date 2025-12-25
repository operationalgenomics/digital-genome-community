use serde::{Serialize, Deserialize};
use super::provenance::FoucaultianTruth;
use std::collections::HashMap;
use uuid::Uuid;

/// Blocknowledge.
/// 
/// A crystallized unit of memory containing a batch of Foucaultian Truths.
/// It represents a "Page" in the history book of the Digital Genome.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blocknowledge {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: i64,
    /// The truths preserved in this block.
    pub content: Vec<FoucaultianTruth>,
    /// The seal of the block.
    pub hash: String,
}

/// The Action Archive.
/// 
/// The in-memory manager of the historical timeline.
/// Enforces the "Append-Only" axiom.
pub struct ActionArchive {
    /// Linearity of blocks.
    pub chain: Vec<Blocknowledge>,
    /// Fast lookup index: TruthId -> Block Index.
    pub index: HashMap<Uuid, u64>,
}

impl ActionArchive {
    pub fn new() -> Self {
        // Genesis Block (The first memory)
        let genesis = Blocknowledge {
            index: 0,
            previous_hash: "000000000000".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            content: Vec::new(),
            hash: "GENESIS_HASH".to_string(),
        };

        Self {
            chain: vec![genesis],
            index: HashMap::new(),
        }
    }

    /// Appends a new Blocknowledge to the history.
    pub fn commit_block(&mut self, truths: Vec<FoucaultianTruth>) -> Result<(), String> {
        if truths.is_empty() {
            return Ok(()); // Nothing to commit
        }

        let last_block = self.chain.last().unwrap();
        let new_index = last_block.index + 1;
        
        // Simple hash simulation for structure
        let new_hash = format!("block_{}_{}", new_index, truths.len());

        let block = Blocknowledge {
            index: new_index,
            previous_hash: last_block.hash.clone(),
            timestamp: chrono::Utc::now().timestamp(),
            content: truths.clone(),
            hash: new_hash,
        };

        // Update Index for fast retrieval
        for truth in &truths {
            self.index.insert(truth.id.0, new_index);
        }

        self.chain.push(block);
        Ok(())
    }

    /// Retrieves a truth by ID (O(1) lookup).
    pub fn get_truth(&self, id: &Uuid) -> Option<&FoucaultianTruth> {
        let block_idx = self.index.get(id)?;
        let block = self.chain.get(*block_idx as usize)?;
        block.content.iter().find(|t| t.id.0 == *id)
    }
}
