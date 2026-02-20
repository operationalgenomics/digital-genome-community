//! Canon v5.1 | GDC v1.0.0β
//!
//! Financial Adapter - Market Data → UNL
//!
//! Canon: LEI-AF-14-01 (Adapter Estrutural Canônico Trans-Kingdom)
//!
//! ## Responsabilidade
//!
//! Mapear dados de mercado financeiro (preços, volumes, transações)
//! para UNL normalizada de forma determinística.
//!
//! ## Schema Financial
//!
//! ```json
//! {
//!   "symbol": "string",
//!   "timestamp": "u64",
//!   "price": "f64",
//!   "volume": "u64",
//!   "bid": "f64",
//!   "ask": "f64",
//!   "market": "string"
//! }
//! ```

use crate::adapter::{
    Adapter, AdapterError, AdapterMetadata, AdapterVersion,
    Domain, FieldDefinition, FieldType, Relation,
    SchemaDefinition, UnlNormalized,
};
use serde::{Deserialize, Serialize};

// =============================================================================
// FINANCIAL SCHEMA
// =============================================================================

/// Financial market data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialMarketData {
    pub symbol: String,
    pub timestamp: u64,
    pub price: f64,
    pub volume: u64,
    pub bid: f64,
    pub ask: f64,
    pub market: String,
}

// =============================================================================
// FINANCIAL ADAPTER
// =============================================================================

/// Financial Market Adapter.
///
/// Mapeia dados de mercado financeiro para UNL normalizada.
///
/// # Canon: LEI-AF-14-01
///
/// ## Mapeamento
///
/// - `symbol` → Token "asset_<symbol>"
/// - `price` → Token "price_<value>"
/// - `volume` → Token "vol_<value>"
/// - `bid/ask` → Tokens "bid_<value>", "ask_<value>"
/// - `market` → Token "market_<name>"
/// - Spread calculado → Relação "has_spread"
///
/// ## Determinismo
///
/// - Mesma entrada JSON → Mesma UNL (sempre)
/// - Não usa clock do sistema
/// - Não acessa API de mercado
pub struct FinancialAdapter {
    version: AdapterVersion,
}

impl FinancialAdapter {
    /// Create new financial adapter.
    pub fn new() -> Self {
        Self {
            version: AdapterVersion::new(1, 0, 0),
        }
    }
}

impl Default for FinancialAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl Adapter for FinancialAdapter {
    fn adapt(&self, input: &[u8]) -> Result<UnlNormalized, AdapterError> {
        // Parse JSON
        let market_data: FinancialMarketData = serde_json::from_slice(input)
            .map_err(|e| AdapterError::InvalidInput(format!("JSON parse error: {}", e)))?;
        
        // Validate required fields
        if market_data.symbol.is_empty() {
            return Err(AdapterError::MissingField("symbol".to_string()));
        }
        
        if market_data.market.is_empty() {
            return Err(AdapterError::MissingField("market".to_string()));
        }
        
        // Map to UNL tokens
        let mut tokens = Vec::new();
        
        // 1. Asset symbol
        tokens.push(format!("asset_{}", normalize_symbol(&market_data.symbol)));
        
        // 2. Price
        tokens.push(format!("price_{:.2}", market_data.price));
        
        // 3. Volume
        tokens.push(format!("vol_{}", market_data.volume));
        
        // 4. Bid
        tokens.push(format!("bid_{:.2}", market_data.bid));
        
        // 5. Ask
        tokens.push(format!("ask_{:.2}", market_data.ask));
        
        // 6. Market
        tokens.push(format!("market_{}", normalize_market(&market_data.market)));
        
        // 7. Timestamp
        tokens.push(format!("ts_{}", market_data.timestamp));
        
        // 8. Spread (derived)
        let spread = market_data.ask - market_data.bid;
        tokens.push(format!("spread_{:.4}", spread));
        
        // Map to relations
        let relations = vec![
            // Asset → Price
            Relation {
                from_token: tokens[0].clone(), // asset
                to_token: tokens[1].clone(),   // price
                relation_type: "has_price".to_string(),
            },
            // Asset → Volume
            Relation {
                from_token: tokens[0].clone(), // asset
                to_token: tokens[2].clone(),   // volume
                relation_type: "has_volume".to_string(),
            },
            // Asset → Bid
            Relation {
                from_token: tokens[0].clone(), // asset
                to_token: tokens[3].clone(),   // bid
                relation_type: "has_bid".to_string(),
            },
            // Asset → Ask
            Relation {
                from_token: tokens[0].clone(), // asset
                to_token: tokens[4].clone(),   // ask
                relation_type: "has_ask".to_string(),
            },
            // Asset → Market
            Relation {
                from_token: tokens[0].clone(), // asset
                to_token: tokens[5].clone(),   // market
                relation_type: "traded_on".to_string(),
            },
            // Timestamp → Asset
            Relation {
                from_token: tokens[6].clone(), // timestamp
                to_token: tokens[0].clone(),   // asset
                relation_type: "timestamp_of".to_string(),
            },
            // Spread → Bid (derived)
            Relation {
                from_token: tokens[7].clone(), // spread
                to_token: tokens[3].clone(),   // bid
                relation_type: "derived_from".to_string(),
            },
            // Spread → Ask (derived)
            Relation {
                from_token: tokens[7].clone(), // spread
                to_token: tokens[4].clone(),   // ask
                relation_type: "derived_from".to_string(),
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
            timestamp_utc: market_data.timestamp, // From market data, not system clock
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
        Domain::Financial
    }
    
    fn schema(&self) -> SchemaDefinition {
        SchemaDefinition {
            name: "FinancialMarket".to_string(),
            version: "1.0.0".to_string(),
            description: "Financial market data schema".to_string(),
            fields: vec![
                FieldDefinition {
                    name: "symbol".to_string(),
                    field_type: FieldType::String,
                    required: true,
                    description: "Asset symbol (e.g., AAPL, BTC-USD)".to_string(),
                },
                FieldDefinition {
                    name: "timestamp".to_string(),
                    field_type: FieldType::Timestamp,
                    required: true,
                    description: "Unix timestamp in seconds".to_string(),
                },
                FieldDefinition {
                    name: "price".to_string(),
                    field_type: FieldType::Float,
                    required: true,
                    description: "Current price".to_string(),
                },
                FieldDefinition {
                    name: "volume".to_string(),
                    field_type: FieldType::Integer,
                    required: true,
                    description: "Trading volume".to_string(),
                },
                FieldDefinition {
                    name: "bid".to_string(),
                    field_type: FieldType::Float,
                    required: true,
                    description: "Bid price".to_string(),
                },
                FieldDefinition {
                    name: "ask".to_string(),
                    field_type: FieldType::Float,
                    required: true,
                    description: "Ask price".to_string(),
                },
                FieldDefinition {
                    name: "market".to_string(),
                    field_type: FieldType::String,
                    required: true,
                    description: "Market name (e.g., NYSE, NASDAQ)".to_string(),
                },
            ],
        }
    }
}

// =============================================================================
// HELPERS
// =============================================================================

/// Normalize symbol (deterministic).
fn normalize_symbol(symbol: &str) -> String {
    symbol.to_uppercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect()
}

/// Normalize market name (deterministic).
fn normalize_market(market: &str) -> String {
    market.to_uppercase()
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_data() -> Vec<u8> {
        let data = FinancialMarketData {
            symbol: "AAPL".to_string(),
            timestamp: 1708300000,
            price: 150.25,
            volume: 1000000,
            bid: 150.20,
            ask: 150.30,
            market: "NASDAQ".to_string(),
        };
        serde_json::to_vec(&data).unwrap()
    }
    
    #[test]
    fn test_financial_adapter_basic() {
        let adapter = FinancialAdapter::new();
        let input = create_test_data();
        
        let unl = adapter.adapt(&input).unwrap();
        
        // Verify tokens
        assert_eq!(unl.tokens.len(), 8);
        assert!(unl.tokens[0].starts_with("asset_"));
        assert!(unl.tokens[1].starts_with("price_"));
        assert!(unl.tokens[2].starts_with("vol_"));
        assert!(unl.tokens[3].starts_with("bid_"));
        assert!(unl.tokens[4].starts_with("ask_"));
        assert!(unl.tokens[5].starts_with("market_"));
        assert!(unl.tokens[6].starts_with("ts_"));
        assert!(unl.tokens[7].starts_with("spread_"));
        
        // Verify relations
        assert_eq!(unl.relations.len(), 8);
    }
    
    #[test]
    fn test_financial_adapter_determinism() {
        // Canon: LEI-AF-14-01 §1
        let adapter = FinancialAdapter::new();
        let input = create_test_data();
        
        let unl1 = adapter.adapt(&input).unwrap();
        let unl2 = adapter.adapt(&input).unwrap();
        
        // MUST be identical
        assert_eq!(unl1.tokens, unl2.tokens);
        assert_eq!(unl1.relations, unl2.relations);
        assert_eq!(unl1.metadata.source_hash, unl2.metadata.source_hash);
    }
    
    #[test]
    fn test_financial_adapter_spread_calculation() {
        let adapter = FinancialAdapter::new();
        
        let data = FinancialMarketData {
            symbol: "BTC-USD".to_string(),
            timestamp: 1708300000,
            price: 50000.0,
            volume: 100,
            bid: 49990.0,
            ask: 50010.0,
            market: "COINBASE".to_string(),
        };
        
        let input = serde_json::to_vec(&data).unwrap();
        let unl = adapter.adapt(&input).unwrap();
        
        // Verify spread token
        assert!(unl.tokens[7].contains("spread_20.0000"));
    }
    
    #[test]
    fn test_financial_adapter_different_inputs() {
        let adapter = FinancialAdapter::new();
        
        let data1 = FinancialMarketData {
            symbol: "AAPL".to_string(),
            timestamp: 1708300000,
            price: 150.0,
            volume: 1000000,
            bid: 149.90,
            ask: 150.10,
            market: "NASDAQ".to_string(),
        };
        
        let data2 = FinancialMarketData {
            symbol: "GOOGL".to_string(),
            timestamp: 1708300001,
            price: 140.0,
            volume: 500000,
            bid: 139.95,
            ask: 140.05,
            market: "NASDAQ".to_string(),
        };
        
        let input1 = serde_json::to_vec(&data1).unwrap();
        let input2 = serde_json::to_vec(&data2).unwrap();
        
        let unl1 = adapter.adapt(&input1).unwrap();
        let unl2 = adapter.adapt(&input2).unwrap();
        
        // MUST be different
        assert_ne!(unl1.tokens, unl2.tokens);
        assert_ne!(unl1.metadata.source_hash, unl2.metadata.source_hash);
    }
}
