//! Canon v5.1 | GDC v1.0.0β
//!
//! Industrial Adapter - Sensor IoT → UNL
//!
//! Canon: LEI-AF-14-01 (Adapter Estrutural Canônico Trans-Kingdom)
//!
//! ## Responsabilidade
//!
//! Mapear dados de sensores industriais (temperatura, pressão, vibração)
//! para UNL normalizada de forma determinística.
//!
//! ## Schema Industrial
//!
//! ```json
//! {
//!   "sensor_id": "string",
//!   "timestamp": "u64",
//!   "temperature": "f32",
//!   "pressure": "f32",
//!   "vibration": "f32",
//!   "status": "string"
//! }
//! ```

use crate::adapter::{
    Adapter, AdapterError, AdapterMetadata, AdapterVersion,
    Domain, FieldDefinition, FieldType, Relation,
    SchemaDefinition, UnlNormalized,
};
use serde::{Deserialize, Serialize};

// =============================================================================
// INDUSTRIAL SCHEMA
// =============================================================================

/// Industrial sensor data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustrialSensorData {
    pub sensor_id: String,
    pub timestamp: u64,
    pub temperature: f32,
    pub pressure: f32,
    pub vibration: f32,
    pub status: String,
}

// =============================================================================
// INDUSTRIAL ADAPTER
// =============================================================================

/// Industrial IoT Adapter.
///
/// Mapeia dados de sensores industriais para UNL normalizada.
///
/// # Canon: LEI-AF-14-01
///
/// ## Mapeamento
///
/// - `sensor_id` → Token "sensor_<id>"
/// - Medições → Tokens "temp_<value>", "press_<value>", "vibr_<value>"
/// - Status → Token "status_<value>"
/// - Timestamp → Relação temporal
///
/// ## Determinismo
///
/// - Mesma entrada JSON → Mesma UNL (sempre)
/// - Não usa clock do sistema
/// - Não acessa rede ou filesystem
pub struct IndustrialAdapter {
    version: AdapterVersion,
}

impl IndustrialAdapter {
    /// Create new industrial adapter.
    pub fn new() -> Self {
        Self {
            version: AdapterVersion::new(1, 0, 0),
        }
    }
}

impl Default for IndustrialAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl Adapter for IndustrialAdapter {
    fn adapt(&self, input: &[u8]) -> Result<UnlNormalized, AdapterError> {
        // Parse JSON
        let sensor_data: IndustrialSensorData = serde_json::from_slice(input)
            .map_err(|e| AdapterError::InvalidInput(format!("JSON parse error: {}", e)))?;
        
        // Validate required fields
        if sensor_data.sensor_id.is_empty() {
            return Err(AdapterError::MissingField("sensor_id".to_string()));
        }
        
        // Map to UNL tokens
        let mut tokens = Vec::new();
        
        // 1. Sensor ID
        tokens.push(format!("sensor_{}", normalize_id(&sensor_data.sensor_id)));
        
        // 2. Temperature
        tokens.push(format!("temp_{:.2}", sensor_data.temperature));
        
        // 3. Pressure
        tokens.push(format!("press_{:.2}", sensor_data.pressure));
        
        // 4. Vibration
        tokens.push(format!("vibr_{:.2}", sensor_data.vibration));
        
        // 5. Status
        tokens.push(format!("status_{}", normalize_status(&sensor_data.status)));
        
        // 6. Timestamp (as token)
        tokens.push(format!("ts_{}", sensor_data.timestamp));
        
        // Map to relations
        let relations = vec![
            // Sensor → Measurements
            Relation {
                from_token: tokens[0].clone(), // sensor
                to_token: tokens[1].clone(),   // temp
                relation_type: "measures".to_string(),
            },
            Relation {
                from_token: tokens[0].clone(), // sensor
                to_token: tokens[2].clone(),   // press
                relation_type: "measures".to_string(),
            },
            Relation {
                from_token: tokens[0].clone(), // sensor
                to_token: tokens[3].clone(),   // vibr
                relation_type: "measures".to_string(),
            },
            // Sensor → Status
            Relation {
                from_token: tokens[0].clone(), // sensor
                to_token: tokens[4].clone(),   // status
                relation_type: "has_status".to_string(),
            },
            // Timestamp → Sensor (temporal)
            Relation {
                from_token: tokens[5].clone(), // timestamp
                to_token: tokens[0].clone(),   // sensor
                relation_type: "timestamp_of".to_string(),
            },
        ];
        
        // Compute source hash (deterministic)
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        let mut source_hash = [0u8; 32];
        source_hash.copy_from_slice(&result);
        
        // Metadata
        let metadata = AdapterMetadata {
            domain: self.domain(),
            adapter_version: self.version(),
            timestamp_utc: sensor_data.timestamp, // From sensor data, not system clock
            source_hash,
        };
        
        Ok(UnlNormalized {
            tokens,
            relations,
            metadata,
        })
    }
    
    fn version(&self) -> AdapterVersion {
        self.version
    }
    
    fn domain(&self) -> Domain {
        Domain::Industrial
    }
    
    fn schema(&self) -> SchemaDefinition {
        SchemaDefinition {
            name: "IndustrialSensor".to_string(),
            version: "1.0.0".to_string(),
            description: "Industrial IoT sensor data schema".to_string(),
            fields: vec![
                FieldDefinition {
                    name: "sensor_id".to_string(),
                    field_type: FieldType::String,
                    required: true,
                    description: "Unique sensor identifier".to_string(),
                },
                FieldDefinition {
                    name: "timestamp".to_string(),
                    field_type: FieldType::Timestamp,
                    required: true,
                    description: "Unix timestamp in seconds".to_string(),
                },
                FieldDefinition {
                    name: "temperature".to_string(),
                    field_type: FieldType::Float,
                    required: true,
                    description: "Temperature in Celsius".to_string(),
                },
                FieldDefinition {
                    name: "pressure".to_string(),
                    field_type: FieldType::Float,
                    required: true,
                    description: "Pressure in bar".to_string(),
                },
                FieldDefinition {
                    name: "vibration".to_string(),
                    field_type: FieldType::Float,
                    required: true,
                    description: "Vibration in mm/s".to_string(),
                },
                FieldDefinition {
                    name: "status".to_string(),
                    field_type: FieldType::String,
                    required: true,
                    description: "Operational status: ok, warning, critical".to_string(),
                },
            ],
        }
    }
}

// =============================================================================
// HELPERS
// =============================================================================

/// Normalize sensor ID (deterministic).
fn normalize_id(id: &str) -> String {
    id.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

/// Normalize status (deterministic).
fn normalize_status(status: &str) -> String {
    status.to_lowercase()
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_data() -> Vec<u8> {
        let data = IndustrialSensorData {
            sensor_id: "SENSOR-001".to_string(),
            timestamp: 1708300000,
            temperature: 75.5,
            pressure: 2.3,
            vibration: 0.12,
            status: "ok".to_string(),
        };
        serde_json::to_vec(&data).unwrap()
    }
    
    #[test]
    fn test_industrial_adapter_basic() {
        let adapter = IndustrialAdapter::new();
        let input = create_test_data();
        
        let unl = adapter.adapt(&input).unwrap();
        
        // Verify tokens
        assert_eq!(unl.tokens.len(), 6);
        assert!(unl.tokens[0].starts_with("sensor_"));
        assert!(unl.tokens[1].starts_with("temp_"));
        assert!(unl.tokens[2].starts_with("press_"));
        assert!(unl.tokens[3].starts_with("vibr_"));
        assert!(unl.tokens[4].starts_with("status_"));
        assert!(unl.tokens[5].starts_with("ts_"));
        
        // Verify relations
        assert_eq!(unl.relations.len(), 5);
    }
    
    #[test]
    fn test_industrial_adapter_determinism() {
        // Canon: LEI-AF-14-01 §1
        let adapter = IndustrialAdapter::new();
        let input = create_test_data();
        
        let unl1 = adapter.adapt(&input).unwrap();
        let unl2 = adapter.adapt(&input).unwrap();
        
        // MUST be identical
        assert_eq!(unl1.tokens, unl2.tokens);
        assert_eq!(unl1.relations, unl2.relations);
        assert_eq!(unl1.metadata.source_hash, unl2.metadata.source_hash);
    }
    
    #[test]
    fn test_industrial_adapter_different_inputs() {
        let adapter = IndustrialAdapter::new();
        
        let data1 = IndustrialSensorData {
            sensor_id: "SENSOR-001".to_string(),
            timestamp: 1708300000,
            temperature: 75.5,
            pressure: 2.3,
            vibration: 0.12,
            status: "ok".to_string(),
        };
        
        let data2 = IndustrialSensorData {
            sensor_id: "SENSOR-002".to_string(),
            timestamp: 1708300001,
            temperature: 80.0,
            pressure: 2.5,
            vibration: 0.15,
            status: "warning".to_string(),
        };
        
        let input1 = serde_json::to_vec(&data1).unwrap();
        let input2 = serde_json::to_vec(&data2).unwrap();
        
        let unl1 = adapter.adapt(&input1).unwrap();
        let unl2 = adapter.adapt(&input2).unwrap();
        
        // MUST be different
        assert_ne!(unl1.tokens, unl2.tokens);
        assert_ne!(unl1.metadata.source_hash, unl2.metadata.source_hash);
    }
    
    #[test]
    fn test_industrial_adapter_missing_field() {
        let adapter = IndustrialAdapter::new();
        
        // Invalid JSON (missing sensor_id)
        let invalid_json = r#"{"timestamp": 123}"#;
        let result = adapter.adapt(invalid_json.as_bytes());
        
        assert!(result.is_err());
    }
}
