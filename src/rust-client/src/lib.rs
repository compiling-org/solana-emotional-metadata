//! # NFT Rust Client
//!
//! Core Rust library for generating and formatting audiovisual/emotional metadata
//! for the Compiling.org creative computing ecosystem.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

// WASM bindings
use wasm_bindgen::prelude::*;

// Re-export modules for WASM usage
pub mod webgpu_engine;
pub mod blockchain_integration;

// Re-export for convenience
pub use webgpu_engine::*;
pub use blockchain_integration::*;

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
}

/// Individual performance parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceParameter {
    pub name: String,
    pub value: f32,
    pub timestamp: DateTime<Utc>,
}

/// Creative session manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeSession {
    session_id: Uuid,
    start_time: DateTime<Utc>,
    data_points: Vec<CreativeData>,
    metadata: HashMap<String, serde_json::Value>,
}

impl CreativeSession {
    /// Create a new creative session
    pub fn new() -> Self {
        Self {
            session_id: Uuid::new_v4(),
            start_time: Utc::now(),
            data_points: Vec::new(),
            metadata: HashMap::new(),
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
            }).collect::<Vec<_>>(),
            "metadata": self.metadata,
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
        });

        Ok(serde_json::to_vec(&export_data)?)
    }
}

/// Generate emotional data from raw inputs
pub fn generate_emotional_data(
    valence: f32,
    arousal: f32,
    dominance: f32,
    raw_vector: Vec<f32>
) -> EmotionalData {
    EmotionalData {
        timestamp: Utc::now(),
        valence: valence.clamp(-1.0, 1.0),
        arousal: arousal.clamp(0.0, 1.0),
        dominance: dominance.clamp(0.0, 1.0),
        confidence: 0.8, // Default confidence
        raw_vector,
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
}