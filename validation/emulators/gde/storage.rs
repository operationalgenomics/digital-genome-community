//! Canon v5.1 | GDC v1.0.0γ
//!
//! GDE Storage - Persistência Real de DNA
//!
//! ## Responsabilidade
//!
//! - Persistir DNA em disco (determinístico)
//! - Recuperar DNA como Σ válido
//! - Manter histórico completo
//! - Validar integridade (checksums)
//!
//! ## Não-Responsabilidades
//!
//! - NÃO processa DNA (responsabilidade do GDC)
//! - NÃO orquestra pipeline (responsabilidade do GDO)
//! - NÃO adapta sinais (responsabilidade dos Adapters)

use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

// =============================================================================
// DNA RECORD - REGISTRO PERSISTIDO
// =============================================================================

/// DNA record stored on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnaRecord {
    /// Unique ID
    pub id: String,
    
    /// DNA bytes (serialized GdQmnInstruction or similar)
    pub dna_bytes: Vec<u8>,
    
    /// Checksum (integrity verification)
    pub checksum: [u8; 32],
    
    /// Timestamp (from DNA metadata, not system)
    pub timestamp: u64,
    
    /// Generation number (cycle count)
    pub generation: u64,
    
    /// Parent DNA ID (for lineage tracking)
    pub parent_id: Option<String>,
}

impl DnaRecord {
    /// Create new DNA record.
    pub fn new(
        id: String,
        dna_bytes: Vec<u8>,
        timestamp: u64,
        generation: u64,
        parent_id: Option<String>,
    ) -> Self {
        let checksum = compute_checksum(&dna_bytes);
        
        Self {
            id,
            dna_bytes,
            checksum,
            timestamp,
            generation,
            parent_id,
        }
    }
    
    /// Verify integrity.
    pub fn verify(&self) -> bool {
        let computed = compute_checksum(&self.dna_bytes);
        computed == self.checksum
    }
}

// =============================================================================
// DNA STORAGE - ENGINE DE PERSISTÊNCIA
// =============================================================================

/// DNA Storage engine.
///
/// Persiste DNA em disco de forma determinística.
///
/// # Structure
///
/// ```text
/// storage_dir/
///   - dna_<id>.json
///   - dna_<id>.json
///   - index.json
/// ```
pub struct DnaStorage {
    /// Storage directory
    storage_dir: PathBuf,
    
    /// In-memory cache
    cache: Vec<DnaRecord>,
}

impl DnaStorage {
    /// Create new storage with directory.
    pub fn new<P: AsRef<Path>>(storage_dir: P) -> Result<Self, StorageError> {
        let storage_dir = storage_dir.as_ref().to_path_buf();
        
        // Create directory if not exists
        fs::create_dir_all(&storage_dir)
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        
        let mut storage = Self {
            storage_dir,
            cache: Vec::new(),
        };
        
        // Load existing records
        storage.load_index()?;
        
        Ok(storage)
    }
    
    /// Persist DNA record.
    pub fn persist(&mut self, record: DnaRecord) -> Result<(), StorageError> {
        // Verify integrity before persisting
        if !record.verify() {
            return Err(StorageError::IntegrityError(
                "Checksum verification failed".to_string()
            ));
        }
        
        // Write to disk
        let filename = format!("dna_{}.json", record.id);
        let filepath = self.storage_dir.join(&filename);
        
        let json = serde_json::to_string_pretty(&record)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;
        
        let mut file = File::create(&filepath)
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        
        file.write_all(json.as_bytes())
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        
        // Add to cache
        self.cache.push(record);
        
        // Update index
        self.save_index()?;
        
        Ok(())
    }
    
    /// Recall DNA by ID.
    pub fn recall(&self, id: &str) -> Result<DnaRecord, StorageError> {
        // Try cache first
        if let Some(record) = self.cache.iter().find(|r| r.id == id) {
            return Ok(record.clone());
        }
        
        // Read from disk
        let filename = format!("dna_{}.json", id);
        let filepath = self.storage_dir.join(&filename);
        
        let mut file = File::open(&filepath)
            .map_err(|_e| StorageError::NotFound(id.to_string()))?;
        
        let mut json = String::new();
        file.read_to_string(&mut json)
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        
        let record: DnaRecord = serde_json::from_str(&json)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;
        
        // Verify integrity
        if !record.verify() {
            return Err(StorageError::IntegrityError(
                format!("DNA {} failed integrity check", id)
            ));
        }
        
        Ok(record)
    }
    
    /// Recall latest DNA.
    pub fn recall_latest(&self) -> Result<DnaRecord, StorageError> {
        self.cache
            .last()
            .cloned()
            .ok_or(StorageError::NotFound("No DNA records".to_string()))
    }
    
    /// Recall DNA by generation.
    pub fn recall_by_generation(&self, generation: u64) -> Result<DnaRecord, StorageError> {
        self.cache
            .iter()
            .find(|r| r.generation == generation)
            .cloned()
            .ok_or(StorageError::NotFound(format!("Generation {}", generation)))
    }
    
    /// List all DNA IDs.
    pub fn list_ids(&self) -> Vec<String> {
        self.cache.iter().map(|r| r.id.clone()).collect()
    }
    
    /// Count total records.
    pub fn count(&self) -> usize {
        self.cache.len()
    }
    
    /// Get lineage (parent → child chain).
    pub fn get_lineage(&self, id: &str) -> Vec<String> {
        let mut lineage = vec![id.to_string()];
        let mut current_id = id.to_string();
        
        // Traverse parents
        while let Ok(record) = self.recall(&current_id) {
            if let Some(parent_id) = record.parent_id {
                lineage.push(parent_id.clone());
                current_id = parent_id;
            } else {
                break;
            }
        }
        
        lineage.reverse();
        lineage
    }
    
    // -------------------------------------------------------------------------
    // PRIVATE HELPERS
    // -------------------------------------------------------------------------
    
    fn load_index(&mut self) -> Result<(), StorageError> {
        let index_path = self.storage_dir.join("index.json");
        
        if !index_path.exists() {
            return Ok(()); // No index yet
        }
        
        let mut file = File::open(&index_path)
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        
        let mut json = String::new();
        file.read_to_string(&mut json)
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        
        let ids: Vec<String> = serde_json::from_str(&json)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;
        
        // Load each record
        for id in ids {
            if let Ok(record) = self.recall(&id) {
                self.cache.push(record);
            }
        }
        
        Ok(())
    }
    
    fn save_index(&self) -> Result<(), StorageError> {
        let index_path = self.storage_dir.join("index.json");
        
        let ids: Vec<String> = self.cache.iter().map(|r| r.id.clone()).collect();
        
        let json = serde_json::to_string_pretty(&ids)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;
        
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&index_path)
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        
        file.write_all(json.as_bytes())
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        
        Ok(())
    }
}

// =============================================================================
// STORAGE ERROR
// =============================================================================

#[derive(Debug, Clone)]
pub enum StorageError {
    IoError(String),
    SerializationError(String),
    NotFound(String),
    IntegrityError(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(msg) => write!(f, "IO error: {}", msg),
            Self::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            Self::NotFound(id) => write!(f, "Not found: {}", id),
            Self::IntegrityError(msg) => write!(f, "Integrity error: {}", msg),
        }
    }
}

impl std::error::Error for StorageError {}

// =============================================================================
// HELPERS
// =============================================================================

fn compute_checksum(data: &[u8]) -> [u8; 32] {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut checksum = [0u8; 32];
    checksum.copy_from_slice(&result);
    checksum
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    fn create_temp_storage() -> DnaStorage {
        let temp_dir = env::temp_dir().join(format!("gde_test_{}", rand::random::<u64>()));
        DnaStorage::new(temp_dir).unwrap()
    }
    
    #[test]
    fn test_storage_persist_and_recall() {
        let mut storage = create_temp_storage();
        
        let record = DnaRecord::new(
            "dna_001".to_string(),
            vec![0xCA, 0xFE, 0xBA, 0xBE],
            1708300000,
            1,
            None,
        );
        
        storage.persist(record.clone()).unwrap();
        
        let recalled = storage.recall("dna_001").unwrap();
        assert_eq!(recalled.dna_bytes, vec![0xCA, 0xFE, 0xBA, 0xBE]);
        assert_eq!(recalled.generation, 1);
    }
    
    #[test]
    fn test_storage_integrity_verification() {
        let mut storage = create_temp_storage();
        
        let record = DnaRecord::new(
            "dna_002".to_string(),
            vec![0xDE, 0xAD],
            1708300000,
            1,
            None,
        );
        
        assert!(record.verify());
        
        storage.persist(record).unwrap();
        
        let recalled = storage.recall("dna_002").unwrap();
        assert!(recalled.verify());
    }
    
    #[test]
    fn test_storage_latest() {
        let mut storage = create_temp_storage();
        
        for i in 1..=5 {
            let record = DnaRecord::new(
                format!("dna_{:03}", i),
                vec![i as u8],
                1708300000 + i,
                i,
                None,
            );
            storage.persist(record).unwrap();
        }
        
        let latest = storage.recall_latest().unwrap();
        assert_eq!(latest.generation, 5);
    }
    
    #[test]
    fn test_storage_lineage() {
        let mut storage = create_temp_storage();
        
        // Generation 1
        let record1 = DnaRecord::new(
            "dna_001".to_string(),
            vec![0x01],
            1708300000,
            1,
            None,
        );
        storage.persist(record1).unwrap();
        
        // Generation 2 (parent: dna_001)
        let record2 = DnaRecord::new(
            "dna_002".to_string(),
            vec![0x02],
            1708300001,
            2,
            Some("dna_001".to_string()),
        );
        storage.persist(record2).unwrap();
        
        // Generation 3 (parent: dna_002)
        let record3 = DnaRecord::new(
            "dna_003".to_string(),
            vec![0x03],
            1708300002,
            3,
            Some("dna_002".to_string()),
        );
        storage.persist(record3).unwrap();
        
        let lineage = storage.get_lineage("dna_003");
        assert_eq!(lineage, vec!["dna_001", "dna_002", "dna_003"]);
    }
}
