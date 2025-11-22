//! # NFT Rust Client
//!
//! Core Rust library for generating and formatting audiovisual/emotional metadata
//! for the Compiling.org creative computing ecosystem.
//! Enhanced with cross-chain capabilities and advanced data processing.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

// WASM bindings
use wasm_bindgen::prelude::*;

// Re-export modules for WASM usage
pub mod webgpu_engine;
pub mod blockchain_integration;
pub mod enhanced_webgpu_engine;
pub mod ai_blockchain_integration;
pub mod enhanced_soulbound;

// Re-export for convenience
pub use webgpu_engine::*;
pub use blockchain_integration::*;
pub use enhanced_webgpu_engine::*;
pub use ai_blockchain_integration::*;
pub use enhanced_soulbound::*;

// WASM initialization
#[wasm_bindgen(start)]
pub fn wasm_init() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        web_sys::console::log_1(&"Rust WASM module initialized!".into());
    }
}

// WASM-exposed functions
#[wasm_bindgen]
pub fn generate_fractal_metadata(fractal_type: &str, zoom: f32, iterations: u32) -> String {
    let metadata = serde_json::json!({
        "type": "fractal",
        "fractal_type": fractal_type,
        "zoom": zoom,
        "iterations": iterations,
        "timestamp": Utc::now().to_rfc3339(),
    });
    metadata.to_string()
}

#[wasm_bindgen]
pub fn compress_emotional_data(valence: f32, arousal: f32, dominance: f32) -> Vec<u8> {
    // 8-bit quantization for compression
    vec![
        ((valence + 1.0) * 127.5) as u8,  // -1 to 1 -> 0 to 255
        (arousal * 255.0) as u8,           // 0 to 1 -> 0 to 255
        (dominance * 255.0) as u8,         // 0 to 1 -> 0 to 255
    ]
}

/// Creative data types that can be tokenized
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreativeData {
    /// Audiovisual performance data
    Audiovisual(AudiovisualData),
    /// Emotional state vectors
    Emotional(EmotionalData),
    /// Shader parameters and states
    Shader(ShaderData),
    /// Live performance metadata
    Performance(PerformanceData),
    /// Cross-chain bridge data
    Bridge(BridgeData),
    /// Reputation tracking data
    Reputation(ReputationData),
}

/// Audiovisual data from creative sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudiovisualData {
    pub timestamp: DateTime<Utc>,
    pub format: String,
    pub data: Vec<u8>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Emotional state data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalData {
    pub timestamp: DateTime<Utc>,
    pub valence: f32,      // pleasure vs displeasure
    pub arousal: f32,      // calm vs excited
    pub dominance: f32,    // controlled vs in-control
    pub confidence: f32,   // certainty of emotional state
    pub raw_vector: Vec<f32>,
    // Enhanced fields
    pub emotional_category: String, // Human-readable emotional category
    pub emotional_trajectory: Vec<EmotionalPoint>, // Historical emotional path
    pub predicted_emotion: Option<Box<EmotionalData>>, // Predicted next emotional state
    pub emotional_complexity: f32, // Complexity of emotional journey
}

/// Point in emotional trajectory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalPoint {
    pub valence: f32,
    pub arousal: f32,
    pub timestamp: DateTime<Utc>,
}

/// Shader parameter data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaderData {
    pub timestamp: DateTime<Utc>,
    pub shader_type: String,
    pub parameters: HashMap<String, f32>,
    pub seed: u64,
    pub iteration_count: u32,
}

/// Live performance metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceData {
    pub session_id: Uuid,
    pub performer_id: String,
    pub start_time: DateTime<Utc>,
    pub duration_ms: u64,
    pub parameters: Vec<PerformanceParameter>,
    // Enhanced fields
    pub emotional_impact: f32, // How emotionally impactful the performance was
    pub creativity_boost: f32, // How much creativity was demonstrated
    pub audience_engagement: f32, // How engaged the audience was
}

/// Individual performance parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceParameter {
    pub name: String,
    pub value: f32,
    pub timestamp: DateTime<Utc>,
}

/// Cross-chain bridge information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeData {
    pub source_chain: String,
    pub target_chain: String,
    pub source_contract: String,
    pub target_contract: String,
    pub bridge_status: String, // "pending", "bridged", "failed"
    pub bridge_timestamp: DateTime<Utc>,
    pub emotional_metadata: Option<EmotionalVector>,
    // Enhanced fields
    pub emotional_preservation: f32, // How well emotional data was preserved (0-1)
    pub bridge_complexity: f32, // Complexity of the bridging operation
    pub cross_chain_emotional_sync: bool, // Whether emotional data is synced across chains
}

/// Creator reputation tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationData {
    pub creator_id: String,
    pub reputation_score: f32,
    pub total_interactions: u64,
    pub last_updated: DateTime<Utc>,
    pub emotional_metrics: Option<EmotionalMetrics>,
    // Enhanced fields
    pub emotional_consistency: f32, // How consistent the creator's emotional expressions are
    pub creativity_score: f32, // Overall creativity rating
    pub community_rank: f32, // Community standing
}

/// Emotional metrics for reputation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalMetrics {
    pub avg_valence: f32,
    pub avg_arousal: f32,
    pub emotional_consistency: f32,
    // Enhanced fields
    pub emotional_range: f32, // Range of emotions expressed
    pub emotional_maturity: f32, // Maturity of emotional expressions
    pub emotional_volatility: f32, // How volatile the emotions are
}

/// Enhanced emotional vector for creative expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalVector {
    pub valence: f32,     // Emotional positivity/negativity (-1 to 1)
    pub arousal: f32,     // Emotional intensity (0 to 1)
    pub dominance: f32,   // Sense of control (0 to 1)
    pub confidence: f32,  // Confidence in emotional assessment (0 to 1)
    pub timestamp: DateTime<Utc>,   // When emotional data was captured
    // Enhanced fields
    pub emotional_category: String, // Human-readable emotional category
    pub emotional_trajectory: Vec<EmotionalPoint>, // Historical emotional path
    pub predicted_emotion: Option<Box<EmotionalVector>>, // Predicted next emotional state
    pub emotional_complexity: f32, // Complexity of emotional journey
}

impl EmotionalVector {
    /// Create new emotional vector with enhanced fields
    pub fn new(valence: f32, arousal: f32, dominance: f32) -> Self {
        let timestamp = Utc::now();
        let category = Self::get_emotional_category(valence, arousal);
        
        Self {
            valence: valence.clamp(-1.0, 1.0),
            arousal: arousal.clamp(0.0, 1.0),
            dominance: dominance.clamp(0.0, 1.0),
            confidence: 0.8, // Default confidence
            timestamp,
            emotional_category: category,
            emotional_trajectory: vec![],
            predicted_emotion: None,
            emotional_complexity: 0.0,
        }
    }
    
    /// Get human-readable emotional category
    pub fn get_emotional_category(valence: f32, arousal: f32) -> String {
        match (valence, arousal) {
            (v, a) if v > 0.5 && a > 0.5 => "Excited".to_string(),
            (v, a) if v > 0.5 && a <= 0.5 => "Happy".to_string(),
            (v, a) if v <= 0.5 && a > 0.5 => "Anxious".to_string(),
            _ => "Calm".to_string(),
        }
    }
    
    /// Add point to emotional trajectory
    pub fn add_trajectory_point(&mut self, valence: f32, arousal: f32) {
        self.emotional_trajectory.push(EmotionalPoint {
            valence,
            arousal,
            timestamp: Utc::now(),
        });
    }
    
    /// Calculate emotional complexity based on trajectory
    pub fn calculate_complexity(&mut self) {
        if self.emotional_trajectory.len() < 2 {
            self.emotional_complexity = 0.0;
            return;
        }
        
        let mut total_distance = 0.0;
        for i in 1..self.emotional_trajectory.len() {
            let prev = &self.emotional_trajectory[i-1];
            let curr = &self.emotional_trajectory[i];
            let distance = ((curr.valence - prev.valence).powi(2) + 
                           (curr.arousal - prev.arousal).powi(2)).sqrt();
            total_distance += distance;
        }
        
        // Normalize by number of points
        self.emotional_complexity = (total_distance / self.emotional_trajectory.len() as f32).clamp(0.0, 1.0);
    }
    
    /// Predict next emotional state
    pub fn predict_next_emotion(&self) -> Option<EmotionalVector> {
        if self.emotional_trajectory.len() < 3 {
            return None;
        }
        
        let len = self.emotional_trajectory.len();
        let latest = &self.emotional_trajectory[len - 1];
        let previous = &self.emotional_trajectory[len - 2];
        let older = &self.emotional_trajectory[len - 3];
        
        // Simple linear extrapolation
        let valence_delta = (latest.valence - previous.valence) * 0.7 + (previous.valence - older.valence) * 0.3;
        let arousal_delta = (latest.arousal - previous.arousal) * 0.7 + (previous.arousal - older.arousal) * 0.3;
        
        Some(EmotionalVector {
            valence: (latest.valence + valence_delta).clamp(-1.0, 1.0),
            arousal: (latest.arousal + arousal_delta).clamp(0.0, 1.0),
            dominance: self.dominance,
            confidence: (self.confidence - 0.1).max(0.0), // Confidence decreases with prediction
            timestamp: Utc::now(),
            emotional_category: EmotionalVector::get_emotional_category(latest.valence + valence_delta, latest.arousal + arousal_delta),
            emotional_trajectory: self.emotional_trajectory.clone(),
            predicted_emotion: None, // Would need recursive handling in a real implementation
            emotional_complexity: self.emotional_complexity,
        })
    }
}

/// Creative session manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeSession {
    session_id: Uuid,
    start_time: DateTime<Utc>,
    data_points: Vec<CreativeData>,
    metadata: HashMap<String, serde_json::Value>,
    // Add cross-chain bridge support
    pub cross_chain_info: Option<BridgeData>,
    // Add reputation tracking
    pub creator_reputation: Option<ReputationData>,
    // Add emotional computing
    pub emotional_profile: Option<EmotionalVector>,
    // Enhanced fields
    pub emotional_complexity: f32, // Overall complexity of emotional journey in session
    pub creativity_index: f32, // Measure of creativity in the session
    pub community_engagement: f32, // How engaging the session was for the community
}

impl CreativeSession {
    /// Create a new creative session
    pub fn new() -> Self {
        Self {
            session_id: Uuid::new_v4(),
            start_time: Utc::now(),
            data_points: Vec::new(),
            metadata: HashMap::new(),
            cross_chain_info: None,
            creator_reputation: None,
            emotional_profile: None,
            emotional_complexity: 0.0,
            creativity_index: 0.0,
            community_engagement: 0.0,
        }
    }

    /// Add creative data to the session
    pub fn add_data(&mut self, data: CreativeData) {
        self.data_points.push(data);
    }

    /// Generate metadata for blockchain tokenization
    pub fn generate_metadata(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let metadata = serde_json::json!({
            "session_id": self.session_id.to_string(),
            "start_time": self.start_time.to_rfc3339(),
            "data_points_count": self.data_points.len(),
            "duration_ms": Utc::now().signed_duration_since(self.start_time).num_milliseconds(),
            "data_types": self.data_points.iter().map(|d| match d {
                CreativeData::Audiovisual(_) => "audiovisual",
                CreativeData::Emotional(_) => "emotional",
                CreativeData::Shader(_) => "shader",
                CreativeData::Performance(_) => "performance",
                CreativeData::Bridge(_) => "bridge",
                CreativeData::Reputation(_) => "reputation",
            }).collect::<Vec<_>>(),
            "metadata": self.metadata,
            // Add cross-chain info if available
            "cross_chain_info": self.cross_chain_info,
            // Add reputation info if available
            "creator_reputation": self.creator_reputation.as_ref().map(|r| serde_json::json!({
                "score": r.reputation_score,
                "interactions": r.total_interactions,
                "emotional_consistency": r.emotional_consistency,
                "creativity_score": r.creativity_score,
                "community_rank": r.community_rank,
            })),
            // Add emotional profile if available
            "emotional_profile": self.emotional_profile,
            // Add enhanced metrics
            "emotional_complexity": self.emotional_complexity,
            "creativity_index": self.creativity_index,
            "community_engagement": self.community_engagement,
        });

        Ok(metadata)
    }

    /// Export session data for IPFS/Filecoin storage
    pub fn export_for_storage(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let export_data = serde_json::json!({
            "session_id": self.session_id.to_string(),
            "start_time": self.start_time.to_rfc3339(),
            "data_points_count": self.data_points.len(),
            "export_timestamp": Utc::now().to_rfc3339(),
            "cross_chain_info": self.cross_chain_info,
            "emotional_profile": self.emotional_profile,
            "emotional_complexity": self.emotional_complexity,
            "creativity_index": self.creativity_index,
        });

        Ok(serde_json::to_vec(&export_data)?)
    }
    
    /// Set cross-chain bridge information
    pub fn set_cross_chain_info(&mut self, info: BridgeData) {
        self.cross_chain_info = Some(info);
    }
    
    /// Set creator reputation
    pub fn set_creator_reputation(&mut self, reputation: ReputationData) {
        self.creator_reputation = Some(reputation);
    }
    
    /// Update reputation based on interaction quality
    pub fn update_reputation(&mut self, interaction_quality: f32) {
        if let Some(ref mut reputation) = self.creator_reputation {
            if interaction_quality > 0.5 {
                reputation.reputation_score = (reputation.reputation_score + 0.1).min(1.0);
            } else {
                reputation.reputation_score = (reputation.reputation_score - 0.05).max(0.0);
            }
            reputation.total_interactions += 1;
            reputation.last_updated = Utc::now();
        }
    }
    
    /// Set emotional profile
    pub fn set_emotional_profile(&mut self, profile: EmotionalVector) {
        self.emotional_profile = Some(profile);
    }
    
    /// Get emotional category based on VAD values
    pub fn get_emotional_category(&self) -> Option<String> {
        if let Some(profile) = &self.emotional_profile {
            Some(match (profile.valence, profile.arousal) {
                (v, a) if v > 0.5 && a > 0.5 => "Excited".to_string(),
                (v, a) if v > 0.5 && a <= 0.5 => "Happy".to_string(),
                (v, a) if v <= 0.5 && a > 0.5 => "Anxious".to_string(),
                _ => "Calm".to_string(),
            })
        } else {
            None
        }
    }
    
    /// Update emotional complexity based on profile
    pub fn update_emotional_complexity(&mut self) {
        if let Some(profile) = &self.emotional_profile {
            self.emotional_complexity = profile.emotional_complexity;
        }
    }
    
    /// Update creativity index based on data points
    pub fn update_creativity_index(&mut self) {
        let mut shader_count = 0;
        let mut emotional_count = 0;
        
        for data in &self.data_points {
            match data {
                CreativeData::Shader(_) => shader_count += 1,
                CreativeData::Emotional(_) => emotional_count += 1,
                _ => {}
            }
        }
        
        // Higher creativity index for more diverse data types
        let diversity_score = if self.data_points.len() > 0 {
            (shader_count as f32 + emotional_count as f32) / self.data_points.len() as f32
        } else {
            0.0
        };
        
        self.creativity_index = diversity_score.clamp(0.0, 1.0);
    }
}

/// Generate emotional data from raw inputs
pub fn generate_emotional_data(
    valence: f32,
    arousal: f32,
    dominance: f32,
    raw_vector: Vec<f32>
) -> EmotionalData {
    let timestamp = Utc::now();
    let category = EmotionalData::get_emotional_category(valence, arousal);
    
    EmotionalData {
        timestamp,
        valence: valence.clamp(-1.0, 1.0),
        arousal: arousal.clamp(0.0, 1.0),
        dominance: dominance.clamp(0.0, 1.0),
        confidence: 0.8, // Default confidence
        raw_vector,
        emotional_category: category,
        emotional_trajectory: vec![],
        predicted_emotion: None,
        emotional_complexity: 0.0,
    }
}

impl EmotionalData {
    /// Get human-readable emotional category
    pub fn get_emotional_category(valence: f32, arousal: f32) -> String {
        match (valence, arousal) {
            (v, a) if v > 0.5 && a > 0.5 => "Excited".to_string(),
            (v, a) if v > 0.5 && a <= 0.5 => "Happy".to_string(),
            (v, a) if v <= 0.5 && a > 0.5 => "Anxious".to_string(),
            _ => "Calm".to_string(),
        }
    }
    
    /// Add point to emotional trajectory
    pub fn add_trajectory_point(&mut self, valence: f32, arousal: f32) {
        self.emotional_trajectory.push(EmotionalPoint {
            valence,
            arousal,
            timestamp: Utc::now(),
        });
    }
    
    /// Calculate emotional complexity based on trajectory
    pub fn calculate_complexity(&mut self) {
        if self.emotional_trajectory.len() < 2 {
            self.emotional_complexity = 0.0;
            return;
        }
        
        let mut total_distance = 0.0;
        for i in 1..self.emotional_trajectory.len() {
            let prev = &self.emotional_trajectory[i-1];
            let curr = &self.emotional_trajectory[i];
            let distance = ((curr.valence - prev.valence).powi(2) + 
                           (curr.arousal - prev.arousal).powi(2)).sqrt();
            total_distance += distance;
        }
        
        // Normalize by number of points
        self.emotional_complexity = (total_distance / self.emotional_trajectory.len() as f32).clamp(0.0, 1.0);
    }
}

/// Generate shader data from parameters
pub fn generate_shader_data(
    shader_type: &str,
    parameters: HashMap<String, f32>,
    seed: u64,
    iteration_count: u32
) -> ShaderData {
    ShaderData {
        timestamp: Utc::now(),
        shader_type: shader_type.to_string(),
        parameters,
        seed,
        iteration_count,
    }
}

/// Utility function to create performance parameter
pub fn create_performance_parameter(name: &str, value: f32) -> PerformanceParameter {
    PerformanceParameter {
        name: name.to_string(),
        value,
        timestamp: Utc::now(),
    }
}

/// Create cross-chain bridge information
pub fn create_bridge_data(
    source_chain: &str,
    target_chain: &str,
    source_contract: &str,
    target_contract: &str,
    emotional_metadata: Option<EmotionalVector>,
) -> BridgeData {
    BridgeData {
        source_chain: source_chain.to_string(),
        target_chain: target_chain.to_string(),
        source_contract: source_contract.to_string(),
        target_contract: target_contract.to_string(),
        bridge_status: "pending".to_string(),
        bridge_timestamp: Utc::now(),
        emotional_metadata,
        emotional_preservation: 0.95, // Default high preservation
        bridge_complexity: 0.3, // Default complexity
        cross_chain_emotional_sync: true, // Default to sync enabled
    }
}

/// Create creator reputation
pub fn create_reputation_data(creator_id: &str) -> ReputationData {
    ReputationData {
        creator_id: creator_id.to_string(),
        reputation_score: 0.5, // Default neutral reputation
        total_interactions: 0,
        last_updated: Utc::now(),
        emotional_metrics: None,
        emotional_consistency: 0.0,
        creativity_score: 0.0,
        community_rank: 0.0,
    }
}

/// Create emotional vector
pub fn create_emotional_vector(valence: f32, arousal: f32, dominance: f32) -> EmotionalVector {
    EmotionalVector::new(valence, arousal, dominance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creative_session_creation() {
        let session = CreativeSession::new();
        assert!(!session.session_id.to_string().is_empty());
        assert!(session.data_points.is_empty());
    }

    #[test]
    fn test_emotional_data_generation() {
        let data = generate_emotional_data(0.5, 0.7, 0.3, vec![0.1, 0.2, 0.3]);
        assert_eq!(data.valence, 0.5);
        assert_eq!(data.arousal, 0.7);
        assert_eq!(data.dominance, 0.3);
    }

    #[test]
    fn test_metadata_generation() {
        let mut session = CreativeSession::new();
        session.add_data(CreativeData::Emotional(generate_emotional_data(0.0, 0.5, 0.5, vec![])));

        let metadata = session.generate_metadata().unwrap();
        assert_eq!(metadata["data_points_count"], 1);
        assert!(metadata["data_types"].as_array().unwrap().contains(&serde_json::json!("emotional")));
    }
    
    #[test]
    fn test_cross_chain_info() {
        let mut session = CreativeSession::new();
        let bridge_data = create_bridge_data("near", "solana", "contract1", "contract2", None);
        session.set_cross_chain_info(bridge_data);
        
        assert!(session.cross_chain_info.is_some());
        assert_eq!(session.cross_chain_info.unwrap().source_chain, "near");
    }
    
    #[test]
    fn test_creator_reputation() {
        let mut session = CreativeSession::new();
        let reputation = create_reputation_data("creator1");
        session.set_creator_reputation(reputation);
        
        assert!(session.creator_reputation.is_some());
        assert_eq!(session.creator_reputation.unwrap().creator_id, "creator1");
    }
    
    #[test]
    fn test_emotional_profile() {
        let mut session = CreativeSession::new();
        let emotional_vector = create_emotional_vector(0.8, 0.9, 0.7);
        session.set_emotional_profile(emotional_vector);
        
        assert!(session.emotional_profile.is_some());
        assert_eq!(session.get_emotional_category().unwrap(), "Excited");
    }
    
    #[test]
    fn test_enhanced_emotional_vector() {
        let mut emotional_vector = EmotionalVector::new(0.7, 0.8, 0.6);
        assert_eq!(emotional_vector.emotional_category, "Excited");
        
        emotional_vector.add_trajectory_point(0.5, 0.4);
        emotional_vector.add_trajectory_point(0.3, 0.2);
        emotional_vector.add_trajectory_point(0.1, 0.6); // Add a third point for prediction
        emotional_vector.calculate_complexity();
        
        assert!(emotional_vector.emotional_complexity >= 0.0);
        assert!(emotional_vector.emotional_complexity <= 1.0);
        
        let prediction = emotional_vector.predict_next_emotion();
        assert!(prediction.is_some());
    }
}