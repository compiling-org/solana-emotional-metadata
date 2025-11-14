//! Neuroemotive Creative Data Accounts
//!
//! Emotionally-aware creative metadata storage with fractal encoding.

/// Emotional state vector (Valence-Arousal-Dominance model)
#[account]
#[derive(Default)]
pub struct EmotionalState {
    pub session_id: [u8; 32],
    pub timestamp: i64,
    pub valence: f32,      // pleasure vs displeasure (-1.0 to 1.0)
    pub arousal: f32,      // calm vs excited (0.0 to 1.0)
    pub dominance: f32,    // controlled vs in-control (0.0 to 1.0)
    pub confidence: f32,   // certainty of measurement (0.0 to 1.0)
    pub source: EmotionSource,
}

/// Source of emotional data
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub enum EmotionSource {
    #[default]
    Manual,
    EEG,
    FacialRecognition,
    VoiceAnalysis,
    PhysiologicalSensors,
    AIInference,
}

/// Stream Diffusion generation tracking
#[account]
#[derive(Default)]
pub struct DiffusionGeneration {
    pub generation_id: [u8; 32],
    pub model_name: String,
    pub prompt: String,
    pub seed: u64,
    pub steps: u32,
    pub guidance_scale: f32,
    pub emotional_conditioning: EmotionalVector,
    pub start_time: i64,
    pub end_time: i64,
    pub result_cid: String, // IPFS CID of generated result
}

/// Compressed emotional vector for efficient storage
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
pub struct EmotionalVector {
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
}

/// Emotional trajectory - sequence of emotional states
#[account]
pub struct EmotionalTrajectory {
    pub trajectory_id: [u8; 32],
    pub creator: Pubkey,
    pub start_time: i64,
    pub compressed_states: Vec<CompressedEmotionalState>,
    pub metadata: TrajectoryMetadata,
}

/// Compressed emotional state for efficient storage
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CompressedEmotionalState {
    pub timestamp_offset: u32, // offset from start_time in milliseconds
    pub v: i8,                  // valence * 100 compressed to i8
    pub a: u8,                  // arousal * 100 compressed to u8
    pub d: u8,                  // dominance * 100 compressed to u8
}

/// Trajectory metadata
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct TrajectoryMetadata {
    pub session_type: String,
    pub participant_count: u8,
    pub environment: String,
    pub tags: Vec<String>,
}

impl EmotionalState {
    /// Calculate emotional distance between two states
    pub fn distance(&self, other: &EmotionalState) -> f32 {
        let dv = self.valence - other.valence;
        let da = self.arousal - other.arousal;
        let dd = self.dominance - other.dominance;
        (dv * dv + da * da + dd * dd).sqrt()
    }

    /// Check if emotional state is valid
    pub fn is_valid(&self) -> bool {
        self.valence >= -1.0 && self.valence <= 1.0
            && self.arousal >= 0.0 && self.arousal <= 1.0
            && self.dominance >= 0.0 && self.dominance <= 1.0
            && self.confidence >= 0.0 && self.confidence <= 1.0
    }

    /// Compress to efficient storage format
    pub fn compress(&self) -> CompressedEmotionalState {
        CompressedEmotionalState {
            timestamp_offset: 0,
            v: (self.valence * 100.0) as i8,
            a: (self.arousal * 100.0) as u8,
            d: (self.dominance * 100.0) as u8,
        }
    }
}

impl CompressedEmotionalState {
    /// Decompress to full emotional state
    pub fn decompress(&self, base_timestamp: i64) -> (i64, f32, f32, f32) {
        (
            base_timestamp + (self.timestamp_offset as i64),
            (self.v as f32) / 100.0,
            (self.a as f32) / 100.0,
            (self.d as f32) / 100.0,
        )
    }
}

impl DiffusionGeneration {
    /// Calculate generation time in seconds
    pub fn generation_time_seconds(&self) -> i64 {
        self.end_time - self.start_time
    }

    /// Check if generation is complete
    pub fn is_complete(&self) -> bool {
        self.end_time > 0 && !self.result_cid.is_empty()
    }
}

/// Neuroemotive session combining emotional data and AI generation
#[account]
pub struct NeuroemotiveSession {
    pub session_id: [u8; 32],
    pub creator: Pubkey,
    pub start_time: i64,
    pub emotional_states: Vec<EmotionalVector>,
    pub diffusion_generations: Vec<[u8; 32]>, // IDs of generation records
    pub stream_active: bool,
    pub total_interactions: u32,
}

impl NeuroemotiveSession {
    /// Add emotional state to session
    pub fn add_emotional_state(&mut self, state: EmotionalVector) {
        self.emotional_states.push(state);
        self.total_interactions += 1;
    }

    /// Calculate average emotional state
    pub fn average_emotional_state(&self) -> EmotionalVector {
        if self.emotional_states.is_empty() {
            return EmotionalVector::default();
        }

        let sum = self.emotional_states.iter().fold(
            EmotionalVector::default(),
            |acc, state| EmotionalVector {
                valence: acc.valence + state.valence,
                arousal: acc.arousal + state.arousal,
                dominance: acc.dominance + state.dominance,
            },
        );

        let count = self.emotional_states.len() as f32;
        EmotionalVector {
            valence: sum.valence / count,
            arousal: sum.arousal / count,
            dominance: sum.dominance / count,
        }
    }

    /// Get emotional variance (measure of emotional volatility)
    pub fn emotional_variance(&self) -> f32 {
        if self.emotional_states.len() < 2 {
            return 0.0;
        }

        let avg = self.average_emotional_state();
        let variance_sum: f32 = self.emotional_states.iter().map(|state| {
            let dv = state.valence - avg.valence;
            let da = state.arousal - avg.arousal;
            let dd = state.dominance - avg.dominance;
            dv * dv + da * da + dd * dd
        }).sum();

        variance_sum / self.emotional_states.len() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotional_state_validation() {
        let valid = EmotionalState {
            valence: 0.5,
            arousal: 0.7,
            dominance: 0.3,
            confidence: 0.9,
            ..Default::default()
        };
        assert!(valid.is_valid());

        let invalid = EmotionalState {
            valence: 2.0, // invalid
            ..Default::default()
        };
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_emotional_compression() {
        let state = EmotionalState {
            valence: 0.5,
            arousal: 0.75,
            dominance: 0.25,
            ..Default::default()
        };

        let compressed = state.compress();
        assert_eq!(compressed.v, 50);
        assert_eq!(compressed.a, 75);
        assert_eq!(compressed.d, 25);
    }

    #[test]
    fn test_emotional_distance() {
        let state1 = EmotionalState {
            valence: 0.0,
            arousal: 0.0,
            dominance: 0.0,
            ..Default::default()
        };

        let state2 = EmotionalState {
            valence: 1.0,
            arousal: 1.0,
            dominance: 1.0,
            ..Default::default()
        };

        let distance = state1.distance(&state2);
        assert!(distance > 0.0);
    }
}
