//! Canon v5.1 | GDC v1.0.0α
//!
//! GDE Educator - Conecta UNL ↔ Representação Humana
//!
//! ## EXTERNAL EMULATOR
//! Implementation is NOT governed by Canon.

use super::bridge::UnlBridge;
use crate::gdo::orchestrator::EdorResult;

// =============================================================================
// GDE EDUCATOR
// =============================================================================

/// GDE Educator - external emulator.
pub struct GdeEducator {
    /// Translation history
    history: Vec<Translation>,
}

/// Translation record.
#[derive(Debug, Clone)]
pub struct Translation {
    pub unl_data: Vec<u8>,
    pub human_repr: String,
}

impl GdeEducator {
    /// Create new GDE educator.
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }
    
    /// Translate UNL to human.
    ///
    /// Canon: Pipeline GDO → GDE → Human
    pub fn translate_to_human(&mut self, edors: Vec<EdorResult>) -> Vec<String> {
        edors
            .into_iter()
            .map(|edor| {
                let human = UnlBridge::unl_to_human(&edor.dna);
                
                self.history.push(Translation {
                    unl_data: edor.dna,
                    human_repr: human.clone(),
                });
                
                human
            })
            .collect()
    }
    
    /// Translate human to UNL.
    ///
    /// Canon: Pipeline Human → GDE → UNL
    pub fn translate_from_human(&mut self, human_input: &str) -> Result<Vec<u8>, String> {
        UnlBridge::human_to_unl(human_input)
            .map_err(|e| format!("{:?}", e))
    }
    
    /// Get translation history count.
    pub fn history_count(&self) -> usize {
        self.history.len()
    }
}

impl Default for GdeEducator {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gde_translates_unl_to_human() {
        // Teste obrigatório v1.0.0α
        let mut educator = GdeEducator::new();
        
        let edors = vec![EdorResult {
            gdc_id: "gdc_1".to_string(),
            dna: vec![0xCA, 0xFE],
        }];
        
        let translations = educator.translate_to_human(edors);
        
        // Canon: GDE traduz UNL para humano
        assert_eq!(translations.len(), 1);
        assert!(translations[0].contains("cafe"));
    }
    
    #[test]
    fn test_gde_translates_human_to_unl() {
        // Teste obrigatório v1.0.0α
        let mut educator = GdeEducator::new();
        
        let human_input = "0xCAFE";
        let unl = educator.translate_from_human(human_input).unwrap();
        
        // Canon: GDE traduz humano para UNL
        assert_eq!(unl, vec![0xCA, 0xFE]);
    }
}
