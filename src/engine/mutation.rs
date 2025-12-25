use uuid::Uuid;
use chrono::Utc;
use crate::core::types::GoldenDna;
use crate::core::identifiers::{DnaId, ActionId, ModelId};
use super::records::MutationRecord;

/// Explicit operations allowed on a Genome.
#[derive(Debug, Clone)]
pub enum CrisprOp {
    /// Replaces the action at `index` with `new_action_id`.
    PointMutation { index: usize, new_action_id: ActionId },
    /// Inserts a sequence of actions at `index`.
    Insertion { index: usize, sequence: Vec<ActionId> },
    /// Deletes a range of actions.
    Deletion { start: usize, end: usize },
}

/// The CRISPR Engine.
/// Handles the surgical modification of Golden DNA.
pub struct CrisprEngine;

impl CrisprEngine {
    /// Applies a mutation to a Golden DNA strand, producing a new version.
    /// Returns the New DNA and the Audit Record.
    pub fn apply(
        original: &GoldenDna, 
        op: CrisprOp, 
        justification: ModelId
    ) -> Result<(GoldenDna, MutationRecord), String> {
        
        let mut new_sequence = original.sequence.clone();
        let op_desc;

        // 1. Execute the Operation (In-Memory)
        match &op {
            CrisprOp::PointMutation { index, new_action_id } => {
                if *index >= new_sequence.len() {
                    return Err("Index out of bounds".to_string());
                }
                new_sequence[*index] = *new_action_id;
                op_desc = format!("POINT_MUTATION at [{}]", index);
            },
            CrisprOp::Insertion { index, sequence } => {
                if *index > new_sequence.len() {
                    return Err("Index out of bounds".to_string());
                }
                // Splice logic
                let mut tail = new_sequence.split_off(*index);
                new_sequence.extend_from_slice(sequence);
                new_sequence.append(&mut tail);
                op_desc = format!("INSERTION of {} items at [{}]", sequence.len(), index);
            },
            CrisprOp::Deletion { start, end } => {
                if *start >= new_sequence.len() || *end > new_sequence.len() || start >= end {
                    return Err("Invalid range".to_string());
                }
                new_sequence.drain(*start..*end);
                op_desc = format!("DELETION range [{}..{}]", start, end);
            }
        }

        // 2. Create the New DNA Struct (Version Bump)
        let new_dna_id = DnaId(Uuid::new_v4());
        let new_dna = GoldenDna {
            id: new_dna_id,
            version: original.version + 1,
            sequence: new_sequence,
            score: 0.0, // Score must be recalculated by Analysis Engine
            target_context: original.target_context,
        };

        // 3. Generate Audit Record
        let record = MutationRecord {
            mutation_id: Uuid::new_v4(),
            original_dna_id: original.id,
            resulting_dna_id: new_dna_id,
            operation_type: op_desc,
            timestamp: Utc::now(),
            justification_model: justification,
        };

        Ok((new_dna, record))
    }
}
