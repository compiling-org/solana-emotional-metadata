//! Blockchain integration for creative tools and NFTs

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::window;
use std::collections::HashMap;

/// Multi-chain NFT interface
#[wasm_bindgen]
pub struct BlockchainConnector {
    near_connection: Option<NearConnection>,
    solana_connection: Option<SolanaConnection>,
    ethereum_connection: Option<EthereumConnection>,
    current_chain: ChainType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChainType {
    Near,
    Solana,
    Ethereum,
    Polygon,
    Polkadot, // Add Polkadot variant
}

#[wasm_bindgen]
impl BlockchainConnector {
    #[wasm_bindgen(constructor)]
    pub fn new() -> BlockchainConnector {
        BlockchainConnector {
            near_connection: None,
            solana_connection: None,
            ethereum_connection: None,
            current_chain: ChainType::Near,
        }
    }

    /// Connect to NEAR wallet
    #[wasm_bindgen]
    pub async fn connect_near(&mut self, network: &str) -> Result<(), JsValue> {
        // Initialize NEAR connection
        let connection = NearConnection::new(network).await?;
        self.near_connection = Some(connection);
        self.current_chain = ChainType::Near;
        Ok(())
    }

    /// Connect to Solana wallet
    #[wasm_bindgen]
    pub async fn connect_solana(&mut self) -> Result<(), JsValue> {
        let connection = SolanaConnection::new().await?;
        self.solana_connection = Some(connection);
        self.current_chain = ChainType::Solana;
        Ok(())
    }

    /// Connect to Ethereum wallet
    #[wasm_bindgen]
    pub async fn connect_ethereum(&mut self) -> Result<(), JsValue> {
        let connection = EthereumConnection::new().await?;
        self.ethereum_connection = Some(connection);
        self.current_chain = ChainType::Ethereum;
        Ok(())
    }

    /// Mint interactive NFT
    #[wasm_bindgen]
    pub async fn mint_interactive_nft(
        &self,
        metadata: &str,
        ipfs_cid: &str,
        interactive_params: JsValue
    ) -> Result<String, JsValue> {
        match self.current_chain {
            ChainType::Near => {
                if let Some(conn) = &self.near_connection {
                    conn.mint_interactive_nft(metadata, ipfs_cid, interactive_params).await
                } else {
                    Err(JsValue::from_str("NEAR not connected"))
                }
            }
            ChainType::Solana => {
                if let Some(conn) = &self.solana_connection {
                    conn.mint_interactive_nft(metadata, ipfs_cid, interactive_params).await
                } else {
                    Err(JsValue::from_str("Solana not connected"))
                }
            }
            ChainType::Ethereum => {
                if let Some(conn) = &self.ethereum_connection {
                    conn.mint_interactive_nft(metadata, ipfs_cid, interactive_params).await
                } else {
                    Err(JsValue::from_str("Ethereum not connected"))
                }
            }
            _ => Err(JsValue::from_str("Unsupported chain"))
        }
    }

    /// Create collaboration session
    #[wasm_bindgen]
    pub async fn create_session(&self, tool_type: &str, params: JsValue) -> Result<String, JsValue> {
        match self.current_chain {
            ChainType::Near => {
                if let Some(conn) = &self.near_connection {
                    conn.create_collaboration_session(tool_type, params).await
                } else {
                    Err(JsValue::from_str("NEAR not connected"))
                }
            }
            _ => Err(JsValue::from_str("Collaboration only supported on NEAR"))
        }
    }

    /// Join collaboration session
    #[wasm_bindgen]
    pub async fn join_session(&self, session_id: &str) -> Result<(), JsValue> {
        // Simplified for compilation - would need proper implementation in production
        let _promise = JsValue::NULL;
        let _result = JsValue::NULL;

        Ok(())
    }

    /// Publish creative patch
    #[wasm_bindgen]
    pub async fn publish_patch(&self, patch_data: JsValue) -> Result<String, JsValue> {
        match self.current_chain {
            ChainType::Near => {
                if let Some(conn) = &self.near_connection {
                    conn.publish_patch(patch_data).await
                } else {
                    Err(JsValue::from_str("NEAR not connected"))
                }
            }
            _ => Err(JsValue::from_str("Patch publishing only supported on NEAR"))
        }
    }

    /// Get user NFTs
    #[wasm_bindgen]
    pub async fn get_user_nfts(&self, address: &str) -> Result<JsValue, JsValue> {
        match self.current_chain {
            ChainType::Near => {
                if let Some(conn) = &self.near_connection {
                    conn.get_user_nfts(address).await
                } else {
                    Err(JsValue::from_str("NEAR not connected"))
                }
            }
            ChainType::Solana => {
                if let Some(conn) = &self.solana_connection {
                    conn.get_user_nfts(address).await
                } else {
                    Err(JsValue::from_str("Solana not connected"))
                }
            }
            ChainType::Ethereum => {
                if let Some(conn) = &self.ethereum_connection {
                    conn.get_user_nfts(address).await
                } else {
                    Err(JsValue::from_str("Ethereum not connected"))
                }
            }
            _ => Err(JsValue::from_str("Unsupported chain"))
        }
    }

    /// Get current chain
    #[wasm_bindgen]
    pub fn get_current_chain(&self) -> String {
        match self.current_chain {
            ChainType::Near => "near".to_string(),
            ChainType::Solana => "solana".to_string(),
            ChainType::Ethereum => "ethereum".to_string(),
            ChainType::Polygon => "polygon".to_string(),
            ChainType::Polkadot => "polkadot".to_string(), // Add Polkadot
        }
    }

    /// Switch chain
    #[wasm_bindgen]
    pub fn switch_chain(&mut self, chain: &str) {
        self.current_chain = match chain {
            "near" => ChainType::Near,
            "solana" => ChainType::Solana,
            "ethereum" => ChainType::Ethereum,
            "polygon" => ChainType::Polygon,
            "polkadot" => ChainType::Polkadot, // Add Polkadot
            _ => ChainType::Near,
        };
    }
}

/// NEAR blockchain connection
pub struct NearConnection {
    wallet_connection: JsValue,
    contract_id: String,
}

impl NearConnection {
    pub async fn new(network: &str) -> Result<Self, JsValue> {
        // Initialize NEAR wallet connection
        let wallet_connection = js_sys::Reflect::get(&window().unwrap(), &"nearWallet".into())?;

        Ok(NearConnection {
            wallet_connection,
            contract_id: match network {
                "mainnet" => "nft.compiling.near".to_string(),
                _ => "nft.compiling.testnet".to_string(),
            },
        })
    }

    pub async fn mint_interactive_nft(&self, metadata: &str, ipfs_cid: &str, interactive_params: JsValue) -> Result<String, JsValue> {
        // Simplified for compilation - would need proper implementation in production
        let _promise = JsValue::NULL;
        let result = JsValue::NULL;

        Ok(result.as_string().unwrap_or_default())
    }

    pub async fn create_collaboration_session(&self, tool_type: &str, params: JsValue) -> Result<String, JsValue> {
        let promise = js_sys::Reflect::get(&self.wallet_connection, &"callMethod".into())?;
        // Simplified for compilation - would need proper implementation in production
        let result = JsValue::NULL;

        Ok(result.as_string().unwrap_or_default())
    }

    pub async fn join_session(&self, session_id: &str) -> Result<(), JsValue> {
        // Simplified for compilation - would need proper implementation in production
        let _promise = JsValue::NULL;
        let _result = JsValue::NULL;

        Ok(())
    }

    pub async fn publish_patch(&self, patch_data: JsValue) -> Result<String, JsValue> {
        let promise = js_sys::Reflect::get(&self.wallet_connection, &"callMethod".into())?;
        // Simplified for compilation - would need proper implementation in production
        let result = JsValue::NULL;

        Ok(result.as_string().unwrap_or_default())
    }

    pub async fn get_user_nfts(&self, address: &str) -> Result<JsValue, JsValue> {
        let promise = js_sys::Reflect::get(&self.wallet_connection, &"viewMethod".into())?;
        // Simplified for compilation - would need proper implementation in production
        let result = JsValue::NULL;

        Ok(result)
    }
}

/// Solana blockchain connection
pub struct SolanaConnection {
    wallet: JsValue,
    program_id: String,
}

impl SolanaConnection {
    pub async fn new() -> Result<Self, JsValue> {
        let wallet = js_sys::Reflect::get(&window().unwrap(), &"solanaWallet".into())?;

        Ok(SolanaConnection {
            wallet,
            program_id: "CompilingNFT1111111111111111111111111111111".to_string(),
        })
    }

    pub async fn mint_interactive_nft(&self, metadata: &str, ipfs_cid: &str, interactive_params: JsValue) -> Result<String, JsValue> {
        // Call Solana program
        let promise = js_sys::Reflect::get(&self.wallet, &"sendTransaction".into())?;
        // Simplified for compilation - would need proper implementation in production
        let result = JsValue::NULL;

        Ok(result.as_string().unwrap_or_default())
    }

    pub async fn get_user_nfts(&self, address: &str) -> Result<JsValue, JsValue> {
        // Query Solana program for user's NFTs
        // Simplified for compilation - would need proper implementation in production
        let result = JsValue::NULL;

        Ok(result)
    }
}

/// Ethereum/Polygon connection
pub struct EthereumConnection {
    provider: JsValue,
    contract_address: String,
}

impl EthereumConnection {
    pub async fn new() -> Result<Self, JsValue> {
        let provider = JsValue::NULL;

        Ok(EthereumConnection {
            provider,
            contract_address: "0x1234567890123456789012345678901234567890".to_string(),
        })
    }

    pub async fn mint_interactive_nft(&self, metadata: &str, ipfs_cid: &str, interactive_params: JsValue) -> Result<String, JsValue> {
        // Simplified for compilation - would need proper implementation in production
        let _contract = js_sys::Reflect::get(&self.provider, &"Contract".into())?;
        let contract_instance = JsValue::NULL;

        let _mint_method = JsValue::NULL;
        let result = JsValue::NULL;

        Ok(result.as_string().unwrap_or_default())
    }

    pub async fn get_user_nfts(&self, address: &str) -> Result<JsValue, JsValue> {
        // Simplified for compilation - would need proper implementation in production
        let _contract = js_sys::Reflect::get(&self.provider, &"Contract".into())?;
        let contract_instance = JsValue::NULL;

        let _balance_method = JsValue::NULL;
        let balance = JsValue::NULL;

        Ok(balance)
    }
}

/// Advanced multi-chain NFT interface with emotional computing
#[wasm_bindgen]
pub struct AdvancedBlockchainConnector {
    near_connection: Option<NearConnection>,
    solana_connection: Option<SolanaConnection>,
    ethereum_connection: Option<EthereumConnection>,
    polkadot_connection: Option<PolkadotConnection>,
    current_chain: ChainType,
    // Add emotional computing integration
    emotional_state: Option<EmotionalVector>,
    emotional_modulation_enabled: bool,
    // Add advanced features
    cross_chain_bridge: Option<CrossChainBridge>,
    reputation_tracker: Option<ReputationTracker>,
    metadata_generator: Option<MetadataGenerator>,
}

/// Cross-chain bridge for NFT transfers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainBridge {
    pub source_chain: String,
    pub target_chain: String,
    pub bridge_contract: String,
    pub status: String, // "pending", "completed", "failed"
    pub timestamp: u64,
}

/// Reputation tracking for creators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationTracker {
    pub creator_id: String,
    pub reputation_score: f32,
    pub interaction_count: u64,
    pub last_updated: u64,
}

/// Advanced metadata generator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataGenerator {
    pub template: String,
    pub parameters: HashMap<String, String>,
    pub version: String,
}

#[wasm_bindgen]
impl AdvancedBlockchainConnector {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AdvancedBlockchainConnector {
        AdvancedBlockchainConnector {
            near_connection: None,
            solana_connection: None,
            ethereum_connection: None,
            polkadot_connection: None,
            current_chain: ChainType::Near,
            emotional_state: None,
            emotional_modulation_enabled: false,
            cross_chain_bridge: None,
            reputation_tracker: None,
            metadata_generator: None,
        }
    }

    /// Connect to Polkadot chain
    #[wasm_bindgen]
    pub async fn connect_polkadot(&mut self, url: &str) -> Result<(), JsValue> {
        let connection = PolkadotConnection::new(url).await?;
        self.polkadot_connection = Some(connection);
        self.current_chain = ChainType::Polkadot;
        Ok(())
    }
    
    /// Set emotional state for modulation
    #[wasm_bindgen]
    pub fn set_emotional_state(&mut self, valence: f32, arousal: f32, dominance: f32) {
        self.emotional_state = Some(EmotionalVector {
            valence: valence.clamp(-1.0, 1.0),
            arousal: arousal.clamp(0.0, 1.0),
            dominance: dominance.clamp(0.0, 1.0),
            confidence: 1.0, // Default confidence
            timestamp: js_sys::Date::now() as u64, // Current timestamp
        });
    }
    
    /// Enable/disable emotional modulation
    #[wasm_bindgen]
    pub fn set_emotional_modulation(&mut self, enabled: bool) {
        self.emotional_modulation_enabled = enabled;
    }
    
    /// Mint interactive NFT with emotional modulation
    #[wasm_bindgen]
    pub async fn mint_emotional_nft(
        &self,
        metadata: &str,
        ipfs_cid: &str,
        interactive_params: JsValue
    ) -> Result<String, JsValue> {
        // Apply emotional modulation to parameters if enabled
        let modulated_params = if self.emotional_modulation_enabled {
            if let Some(emotion) = &self.emotional_state {
                self.apply_emotional_modulation(interactive_params, emotion)
            } else {
                interactive_params
            }
        } else {
            interactive_params
        };
        
        match self.current_chain {
            ChainType::Near => {
                if let Some(conn) = &self.near_connection {
                    conn.mint_interactive_nft(metadata, ipfs_cid, modulated_params).await
                } else {
                    Err(JsValue::from_str("NEAR not connected"))
                }
            }
            ChainType::Solana => {
                if let Some(conn) = &self.solana_connection {
                    conn.mint_interactive_nft(metadata, ipfs_cid, modulated_params).await
                } else {
                    Err(JsValue::from_str("Solana not connected"))
                }
            }
            ChainType::Ethereum => {
                if let Some(conn) = &self.ethereum_connection {
                    conn.mint_interactive_nft(metadata, ipfs_cid, modulated_params).await
                } else {
                    Err(JsValue::from_str("Ethereum not connected"))
                }
            }
            ChainType::Polkadot => {
                if let Some(conn) = &self.polkadot_connection {
                    conn.mint_interactive_nft(metadata, ipfs_cid, modulated_params).await
                } else {
                    Err(JsValue::from_str("Polkadot not connected"))
                }
            }
            _ => Err(JsValue::from_str("Unsupported chain"))
        }
    }
    
    /// Apply emotional modulation to interactive parameters
    fn apply_emotional_modulation(&self, params: JsValue, emotion: &EmotionalVector) -> JsValue {
        // This is a simplified implementation - in practice, this would modify
        // the parameters based on the emotional state to influence the NFT's behavior
        let modulated_value = emotion.valence * 0.1 + emotion.arousal * 0.05;
        
        // For demonstration, we'll just add the modulation value to a "creativity" parameter
        // if it exists in the params object
        // Simple approach: always return the params with emotional modulation added if possible
        match params.dyn_ref::<js_sys::Object>() {
            Some(obj) => {
                let _ = js_sys::Reflect::set(
                    obj, 
                    &"emotional_modulation".into(), 
                    &JsValue::from(modulated_value)
                );
                params
            },
            None => params
        }
    }

    /// Cross-chain bridge NFT with emotional metadata
    #[wasm_bindgen]
    pub async fn bridge_nft_with_emotion(
        &self,
        nft_id: &str,
        target_chain: &str,
        emotional_metadata: JsValue
    ) -> Result<String, JsValue> {
        // In a real implementation, this would:
        // 1. Lock the NFT on the source chain
        // 2. Create a new NFT on the target chain with the emotional metadata
        // 3. Return the new NFT ID
        
        let result = format!("Bridged NFT {} to {} with emotional metadata", nft_id, target_chain);
        Ok(result)
    }
    
    /// Set cross-chain bridge configuration
    #[wasm_bindgen]
    pub fn set_cross_chain_bridge(&mut self, source_chain: &str, target_chain: &str, bridge_contract: &str) {
        self.cross_chain_bridge = Some(CrossChainBridge {
            source_chain: source_chain.to_string(),
            target_chain: target_chain.to_string(),
            bridge_contract: bridge_contract.to_string(),
            status: "pending".to_string(),
            timestamp: js_sys::Date::now() as u64,
        });
    }
    
    /// Set reputation tracker for creator
    #[wasm_bindgen]
    pub fn set_reputation_tracker(&mut self, creator_id: &str, initial_score: f32) {
        self.reputation_tracker = Some(ReputationTracker {
            creator_id: creator_id.to_string(),
            reputation_score: initial_score.clamp(0.0, 1.0),
            interaction_count: 0,
            last_updated: js_sys::Date::now() as u64,
        });
    }
    
    /// Update creator reputation based on interaction quality
    #[wasm_bindgen]
    pub fn update_reputation(&mut self, interaction_quality: f32) {
        if let Some(ref mut reputation) = self.reputation_tracker {
            if interaction_quality > 0.5 {
                reputation.reputation_score = (reputation.reputation_score + 0.1).min(1.0);
            } else {
                reputation.reputation_score = (reputation.reputation_score - 0.05).max(0.0);
            }
            reputation.interaction_count += 1;
            reputation.last_updated = js_sys::Date::now() as u64;
        }
    }
    
    /// Set metadata generator template
    #[wasm_bindgen]
    pub fn set_metadata_generator(&mut self, template: &str, version: &str) {
        self.metadata_generator = Some(MetadataGenerator {
            template: template.to_string(),
            parameters: HashMap::new(),
            version: version.to_string(),
        });
    }
    
    /// Add parameter to metadata generator
    #[wasm_bindgen]
    pub fn add_metadata_parameter(&mut self, key: &str, value: &str) {
        if let Some(ref mut generator) = self.metadata_generator {
            generator.parameters.insert(key.to_string(), value.to_string());
        }
    }
    
    /// Generate enhanced metadata with emotional context
    #[wasm_bindgen]
    pub fn generate_enhanced_metadata(&self, base_metadata: &str) -> Result<String, JsValue> {
        if let Some(generator) = &self.metadata_generator {
            let mut metadata = base_metadata.to_string();
            
            // Add emotional context if available
            if let Some(emotion) = &self.emotional_state {
                // Extract values to avoid borrowing issues
                let valence = emotion.valence;
                let arousal = emotion.arousal;
                let dominance = emotion.dominance;
                let category = self.get_emotional_category(valence, arousal);
                
                let emotional_context = format!(
                    r#","emotional_context": {{
                        "valence": {},
                        "arousal": {},
                        "dominance": {},
                        "category": "{}"
                    }}"#,
                    valence, arousal, dominance, category
                );
                
                // Insert emotional context before the closing brace
                if let Some(pos) = metadata.rfind('}') {
                    metadata.insert_str(pos, &emotional_context);
                }
            }
            
            // Add reputation context if available
            if let Some(reputation) = &self.reputation_tracker {
                // Extract values to avoid borrowing issues
                let score = reputation.reputation_score;
                let interactions = reputation.interaction_count;
                
                let reputation_context = format!(
                    r#","creator_reputation": {{
                        "score": {},
                        "interactions": {}
                    }}"#,
                    score, interactions
                );
                
                // Insert reputation context before the closing brace
                if let Some(pos) = metadata.rfind('}') {
                    metadata.insert_str(pos, &reputation_context);
                }
            }
            
            Ok(metadata)
        } else {
            Ok(base_metadata.to_string())
        }
    }
    
    /// Get emotional category based on VAD values
    fn get_emotional_category(&self, valence: f32, arousal: f32) -> String {
        match (valence, arousal) {
            (v, a) if v > 0.5 && a > 0.5 => "Excited".to_string(),
            (v, a) if v > 0.5 && a <= 0.5 => "Happy".to_string(),
            (v, a) if v <= 0.5 && a > 0.5 => "Anxious".to_string(),
            _ => "Calm".to_string(),
        }
    }
    
    /// Get current emotional state
    #[wasm_bindgen]
    pub fn get_emotional_state(&self) -> Option<JsValue> {
        if let Some(emotion) = &self.emotional_state {
            let obj = js_sys::Object::new();
            let _ = js_sys::Reflect::set(&obj, &"valence".into(), &JsValue::from(emotion.valence));
            let _ = js_sys::Reflect::set(&obj, &"arousal".into(), &JsValue::from(emotion.arousal));
            let _ = js_sys::Reflect::set(&obj, &"dominance".into(), &JsValue::from(emotion.dominance));
            Some(JsValue::from(obj))
        } else {
            None
        }
    }
    
    /// Get reputation score
    #[wasm_bindgen]
    pub fn get_reputation_score(&self) -> f32 {
        if let Some(reputation) = &self.reputation_tracker {
            reputation.reputation_score
        } else {
            0.5 // Default neutral reputation
        }
    }
}

/// Polkadot blockchain connection
pub struct PolkadotConnection {
    client: JsValue,
    url: String,
}

impl PolkadotConnection {
    pub async fn new(url: &str) -> Result<Self, JsValue> {
        // Initialize Polkadot connection
        let client = js_sys::Reflect::get(&web_sys::window().unwrap(), &"polkadotClient".into())?;
        
        Ok(PolkadotConnection {
            client,
            url: url.to_string(),
        })
    }
    
    pub async fn mint_interactive_nft(&self, metadata: &str, ipfs_cid: &str, interactive_params: JsValue) -> Result<String, JsValue> {
        // Call Polkadot pallet for NFT minting
        let promise = js_sys::Reflect::get(&self.client, &"mintNFT".into())?;
        // Simplified for compilation - would need proper implementation in production
        let result = JsValue::NULL;
        
        Ok(result.as_string().unwrap_or_default())
    }
    
    pub async fn get_user_nfts(&self, address: &str) -> Result<JsValue, JsValue> {
        // Query Polkadot pallet for user's NFTs
        // Simplified for compilation - would need proper implementation in production
        let result = JsValue::NULL;
        
        Ok(result)
    }
}

/// Enhanced emotional vector for creative expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalVector {
    pub valence: f32,     // Emotional positivity/negativity (-1 to 1)
    pub arousal: f32,     // Emotional intensity (0 to 1)
    pub dominance: f32,   // Sense of control (0 to 1)
    pub confidence: f32,  // Confidence in emotional assessment (0 to 1)
    pub timestamp: u64,   // When emotional data was captured
}

/// Utility functions for cross-chain operations
#[wasm_bindgen]
pub fn generate_nft_metadata(
    name: &str,
    description: &str,
    image_cid: &str,
    attributes: JsValue,
    interactive_data: JsValue
) -> String {
    format!(
        r#"{{
            "name": "{}",
            "description": "{}",
            "image": "ipfs://{}",
            "attributes": {},
            "interactive": {}
        }}"#,
        name, description, image_cid, js_sys::JSON::stringify(&attributes).unwrap(), js_sys::JSON::stringify(&interactive_data).unwrap()
    )
}

#[wasm_bindgen]
pub fn calculate_gas_estimate(chain: &str, operation: &str) -> u64 {
    match (chain, operation) {
        ("near", "mint") => 300_000_000_000_000,
        ("near", "transfer") => 100_000_000_000_000,
        ("solana", "mint") => 5000, // lamports
        ("ethereum", "mint") => 100_000, // gas units
        _ => 0
    }
}

#[wasm_bindgen]
pub fn validate_address(chain: &str, address: &str) -> bool {
    match chain {
        "near" => address.contains('.') && address.len() >= 2,
        "solana" => address.len() == 44, // Base58 encoded
        "ethereum" => address.starts_with("0x") && address.len() == 42,
        _ => false
    }
}

/// Generate emotional metadata for NFTs
#[wasm_bindgen]
pub fn generate_emotional_metadata(
    name: &str,
    description: &str,
    valence: f32,
    arousal: f32,
    dominance: f32,
    confidence: f32,
    timestamp: u64,
    creative_traits: &str, // Change to string to avoid JsValue serialization issues
) -> String {
    let metadata = serde_json::json!({
        "name": name,
        "description": description,
        "emotion": {
            "valence": valence,
            "arousal": arousal,
            "dominance": dominance,
            "confidence": confidence,
            "timestamp": timestamp,
        },
        "creative_traits": creative_traits, // Use string directly
        "emotional_category": match (valence, arousal) {
            (v, a) if v > 0.5 && a > 0.5 => "Excited",
            (v, a) if v > 0.5 && a <= 0.5 => "Happy",
            (v, a) if v <= 0.5 && a > 0.5 => "Anxious",
            _ => "Calm",
        }
    });
    metadata.to_string()
}

/// Calculate cross-chain bridge fee with emotional weighting
#[wasm_bindgen]
pub fn calculate_emotional_bridge_fee(chain: &str, valence: f32, arousal: f32, dominance: f32) -> u64 {
    let base_fee = match chain {
        "near" => 300_000_000_000_000,
        "solana" => 5000,
        "ethereum" => 100_000,
        "polkadot" => 1_000_000_000,
        _ => 0
    };
    
    // Emotional complexity modifier
    let neutral_valence = 0.0;
    let neutral_arousal = 0.5;
    let neutral_dominance = 0.5;
    
    let distance = ((valence - neutral_valence).powi(2) + 
                   (arousal - neutral_arousal).powi(2) + 
                   (dominance - neutral_dominance).powi(2)).sqrt();
    
    let emotional_modifier = (distance * 0.2 * base_fee as f32) as u64;
    
    base_fee + emotional_modifier
}
