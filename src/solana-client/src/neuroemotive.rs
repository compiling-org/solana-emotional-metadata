//! Neuroemotive Creative Data Accounts
//!
//! Emotionally-aware creative metadata storage with fractal encoding.
//! Enhanced with emotional trajectory analysis and prediction capabilities.

use anchor_lang::prelude::*;
use std::collections::HashMap;

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
    // Add prediction capabilities
    pub predicted_next_state: Option<EmotionalVector>,
    pub prediction_confidence: f32,
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
    // Add emotional pattern analysis
    pub dominant_emotions: Vec<String>,
    pub emotional_volatility: f32,
}

/// Emotional pattern analysis
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct EmotionalPattern {
    pub pattern_id: [u8; 32],
    pub creator: Pubkey,
    pub pattern_type: PatternType,
    pub frequency: u32,
    pub last_occurrence: i64,
    pub associated_traits: Vec<String>,
}

/// Types of emotional patterns
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub enum PatternType {
    #[default]
    Cyclical,
    Ascending,
    Descending,
    Stable,
    Volatile,
}

/// Advanced emotional pattern analysis with machine learning capabilities
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct AdvancedEmotionalPattern {
    pub pattern_id: [u8; 32],
    pub creator: Pubkey,
    pub pattern_type: PatternType,
    pub frequency: u32,
    pub last_occurrence: i64,
    pub associated_traits: Vec<String>,
    // Enhanced fields
    pub emotional_intensity: f32,
    pub temporal_stability: f32,
    pub cross_session_consistency: f32,
    pub predictive_confidence: f32,
    // Add ML-based features
    pub pattern_complexity: f32,
    pub emotional_resonance: f32,
    pub creative_potential: f32,
}

/// Emotional trajectory predictor with advanced ML models
pub struct EmotionalTrajectoryPredictor {
    historical_data: Vec<EmotionalVector>,
    model_parameters: Vec<f32>,
    // Add advanced features
    confidence_history: Vec<f32>,
    prediction_accuracy: f32,
}

impl EmotionalTrajectoryPredictor {
    pub fn new() -> Self {
        Self {
            historical_data: Vec::new(),
            model_parameters: Vec::new(),
            confidence_history: Vec::new(),
            prediction_accuracy: 0.0,
        }
    }
    
    /// Add emotional data point to history
    pub fn add_data_point(&mut self, emotion: EmotionalVector) {
        self.historical_data.push(emotion);
        // Keep only last 100 data points for performance
        if self.historical_data.len() > 100 {
            self.historical_data.remove(0);
        }
    }
    
    /// Train predictive model based on historical data
    pub fn train_model(&mut self) {
        if self.historical_data.len() < 5 {
            return;
        }
        
        // Simple linear regression for each dimension
        let n = self.historical_data.len() as f32;
        let mut sum_x = 0.0;
        let mut sum_y_valence = 0.0;
        let mut sum_y_arousal = 0.0;
        let mut sum_y_dominance = 0.0;
        let mut sum_xy_valence = 0.0;
        let mut sum_xy_arousal = 0.0;
        let mut sum_xy_dominance = 0.0;
        let mut sum_xx = 0.0;
        
        for (i, emotion) in self.historical_data.iter().enumerate() {
            let x = i as f32;
            sum_x += x;
            sum_y_valence += emotion.valence;
            sum_y_arousal += emotion.arousal;
            sum_y_dominance += emotion.dominance;
            sum_xy_valence += x * emotion.valence;
            sum_xy_arousal += x * emotion.arousal;
            sum_xy_dominance += x * emotion.dominance;
            sum_xx += x * x;
        }
        
        let slope_valence = (n * sum_xy_valence - sum_x * sum_y_valence) / (n * sum_xx - sum_x * sum_x);
        let slope_arousal = (n * sum_xy_arousal - sum_x * sum_y_arousal) / (n * sum_xx - sum_x * sum_x);
        let slope_dominance = (n * sum_xy_dominance - sum_x * sum_y_dominance) / (n * sum_xx - sum_x * sum_x);
        
        let intercept_valence = (sum_y_valence - slope_valence * sum_x) / n;
        let intercept_arousal = (sum_y_arousal - slope_arousal * sum_x) / n;
        let intercept_dominance = (sum_y_dominance - slope_dominance * sum_x) / n;
        
        self.model_parameters = vec![slope_valence, intercept_valence, slope_arousal, intercept_arousal, slope_dominance, intercept_dominance];
    }
    
    /// Predict next emotional state
    pub fn predict_next(&self, steps_ahead: u32) -> Option<EmotionalVector> {
        if self.model_parameters.len() != 6 || self.historical_data.is_empty() {
            return None;
        }
        
        let slope_valence = self.model_parameters[0];
        let intercept_valence = self.model_parameters[1];
        let slope_arousal = self.model_parameters[2];
        let intercept_arousal = self.model_parameters[3];
        let slope_dominance = self.model_parameters[4];
        let intercept_dominance = self.model_parameters[5];
        
        let next_x = (self.historical_data.len() as f32) + (steps_ahead as f32);
        
        Some(EmotionalVector {
            valence: (intercept_valence + slope_valence * next_x).clamp(-1.0, 1.0),
            arousal: (intercept_arousal + slope_arousal * next_x).clamp(0.0, 1.0),
            dominance: (intercept_dominance + slope_dominance * next_x).clamp(0.0, 1.0),
        })
    }
    
    /// Calculate prediction confidence based on model fit
    pub fn prediction_confidence(&self) -> f32 {
        if self.model_parameters.len() != 6 || self.historical_data.len() < 3 {
            return 0.0;
        }
        
        // Calculate R-squared as confidence measure
        let mean_valence: f32 = self.historical_data.iter().map(|e| e.valence).sum::<f32>() / self.historical_data.len() as f32;
        let mean_arousal: f32 = self.historical_data.iter().map(|e| e.arousal).sum::<f32>() / self.historical_data.len() as f32;
        let mean_dominance: f32 = self.historical_data.iter().map(|e| e.dominance).sum::<f32>() / self.historical_data.len() as f32;
        
        let mut ss_res_valence = 0.0;
        let mut ss_tot_valence = 0.0;
        let mut ss_res_arousal = 0.0;
        let mut ss_tot_arousal = 0.0;
        let mut ss_res_dominance = 0.0;
        let mut ss_tot_dominance = 0.0;
        
        for (i, emotion) in self.historical_data.iter().enumerate() {
            let x = i as f32;
            let pred_valence = self.model_parameters[1] + self.model_parameters[0] * x;
            let pred_arousal = self.model_parameters[3] + self.model_parameters[2] * x;
            let pred_dominance = self.model_parameters[5] + self.model_parameters[4] * x;
            
            ss_res_valence += (emotion.valence - pred_valence).powi(2);
            ss_tot_valence += (emotion.valence - mean_valence).powi(2);
            ss_res_arousal += (emotion.arousal - pred_arousal).powi(2);
            ss_tot_arousal += (emotion.arousal - mean_arousal).powi(2);
            ss_res_dominance += (emotion.dominance - pred_dominance).powi(2);
            ss_tot_dominance += (emotion.dominance - mean_dominance).powi(2);
        }
        
        let r2_valence = 1.0 - (ss_res_valence / ss_tot_valence);
        let r2_arousal = 1.0 - (ss_res_arousal / ss_tot_arousal);
        let r2_dominance = 1.0 - (ss_res_dominance / ss_tot_dominance);
        
        // Average R-squared as confidence (0-1 scale)
        ((r2_valence + r2_arousal + r2_dominance) / 3.0).max(0.0).min(1.0)
    }
    
    /// Update prediction accuracy based on actual vs predicted values
    pub fn update_accuracy(&mut self, actual: EmotionalVector, predicted: EmotionalVector) {
        let distance = ((actual.valence - predicted.valence).powi(2) +
                       (actual.arousal - predicted.arousal).powi(2) +
                       (actual.dominance - predicted.dominance).powi(2)).sqrt();
        
        // Convert distance to accuracy (0-1 scale)
        let accuracy = 1.0 - distance.min(1.0);
        self.confidence_history.push(accuracy);
        
        // Keep only last 10 accuracy measurements
        if self.confidence_history.len() > 10 {
            self.confidence_history.remove(0);
        }
        
        // Calculate average accuracy
        if !self.confidence_history.is_empty() {
            self.prediction_accuracy = self.confidence_history.iter().sum::<f32>() / self.confidence_history.len() as f32;
        }
    }
    
    /// Get model complexity score
    pub fn model_complexity(&self) -> f32 {
        // Simple complexity measure based on data variance and model parameters
        if self.historical_data.len() < 2 {
            return 0.0;
        }
        
        let mut variance = 0.0;
        let len = self.historical_data.len() as f32;
        
        // Calculate mean
        let mean_valence: f32 = self.historical_data.iter().map(|e| e.valence).sum::<f32>() / len;
        let mean_arousal: f32 = self.historical_data.iter().map(|e| e.arousal).sum::<f32>() / len;
        let mean_dominance: f32 = self.historical_data.iter().map(|e| e.dominance).sum::<f32>() / len;
        
        // Calculate variance
        for emotion in &self.historical_data {
            variance += (emotion.valence - mean_valence).powi(2);
            variance += (emotion.arousal - mean_arousal).powi(2);
            variance += (emotion.dominance - mean_dominance).powi(2);
        }
        
        variance /= len * 3.0; // Normalize by number of dimensions
        variance.min(1.0) // Clamp to 0.0-1.0 range
    }
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
    
    /// Get emotional category based on VAD values
    pub fn get_emotional_category(&self) -> String {
        match (self.valence, self.arousal, self.dominance) {
            (v, a, d) if v > 0.5 && a > 0.5 && d > 0.5 => "Excited".to_string(),
            (v, a, d) if v > 0.5 && a > 0.5 && d <= 0.5 => "Happy".to_string(),
            (v, a, d) if v > 0.5 && a <= 0.5 && d > 0.5 => "Proud".to_string(),
            (v, a, d) if v > 0.5 && a <= 0.5 && d <= 0.5 => "Calm".to_string(),
            (v, a, d) if v <= 0.5 && a > 0.5 && d > 0.5 => "Angry".to_string(),
            (v, a, d) if v <= 0.5 && a > 0.5 && d <= 0.5 => "Anxious".to_string(),
            (v, a, d) if v <= 0.5 && a <= 0.5 && d > 0.5 => "Bored".to_string(),
            _ => "Sad".to_string(),
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
    // Add pattern recognition
    pub recognized_patterns: Vec<[u8; 32]>, // IDs of pattern records
    pub session_traits: HashMap<String, f32>, // Session-specific emotional traits
    // Add advanced analytics
    pub emotional_complexity: f32,
    pub creativity_index: f32,
    pub engagement_score: f32,
    pub predictive_model: Option<EmotionalTrajectoryPredictor>,
}

impl NeuroemotiveSession {
    /// Add emotional state to session
    pub fn add_emotional_state(&mut self, state: EmotionalVector) {
        self.emotional_states.push(state);
        self.total_interactions += 1;
        
        // Update emotional complexity
        self.emotional_complexity = self.calculate_emotional_complexity();
        
        // Update creativity index based on emotional variance
        self.creativity_index = self.calculate_creativity_index();
    }
    
    /// Add emotional state with pattern recognition
    pub fn add_emotional_state_with_pattern(&mut self, state: EmotionalVector) {
        self.add_emotional_state(state);
        
        // Update pattern recognition if we have enough data
        if self.emotional_states.len() >= 5 {
            let patterns = self.identify_advanced_patterns();
            // In a real implementation, we would store these patterns
            // For now, we'll just acknowledge they exist
        }
        
        // Update predictive model if it exists
        if let Some(ref mut model) = self.predictive_model {
            model.add_data_point(state);
            model.train_model();
        }
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
    
    /// Calculate emotional complexity based on variance and trajectory
    pub fn calculate_emotional_complexity(&self) -> f32 {
        if self.emotional_states.len() < 2 {
            return 0.0;
        }
        
        // Use variance as the base complexity measure
        let variance = self.emotional_variance();
        
        // Add trajectory complexity (direction changes)
        let mut direction_changes = 0;
        for i in 1..self.emotional_states.len() {
            let prev = &self.emotional_states[i-1];
            let curr = &self.emotional_states[i];
            
            // Check for significant direction changes in valence
            if (curr.valence - prev.valence).abs() > 0.3 {
                direction_changes += 1;
            }
        }
        
        let trajectory_complexity = direction_changes as f32 / self.emotional_states.len() as f32;
        
        // Combine variance and trajectory complexity (0-1 scale)
        (variance + trajectory_complexity).min(1.0)
    }
    
    /// Calculate creativity index based on emotional dynamics
    pub fn calculate_creativity_index(&self) -> f32 {
        if self.emotional_states.len() < 2 {
            return 0.5; // Default neutral creativity
        }
        
        // Creativity is higher with moderate variance and complexity
        let variance = self.emotional_variance();
        let complexity = self.emotional_complexity;
        
        // Optimal creativity is around variance = 0.3-0.6
        let variance_score = 1.0 - ((variance - 0.45).abs() / 0.45).min(1.0);
        
        // Combine with complexity (0-1 scale)
        (variance_score * 0.7 + complexity * 0.3).min(1.0)
    }
    
    /// Predict next emotional state using simple linear regression
    pub fn predict_next_state(&self) -> Option<EmotionalVector> {
        if self.emotional_states.len() < 3 {
            return None;
        }
        
        let n = self.emotional_states.len() as f32;
        let mut sum_x = 0.0;
        let mut sum_y_valence = 0.0;
        let mut sum_y_arousal = 0.0;
        let mut sum_y_dominance = 0.0;
        let mut sum_xy_valence = 0.0;
        let mut sum_xy_arousal = 0.0;
        let mut sum_xy_dominance = 0.0;
        let mut sum_xx = 0.0;
        
        for (i, state) in self.emotional_states.iter().enumerate() {
            let x = i as f32;
            sum_x += x;
            sum_y_valence += state.valence;
            sum_y_arousal += state.arousal;
            sum_y_dominance += state.dominance;
            sum_xy_valence += x * state.valence;
            sum_xy_arousal += x * state.arousal;
            sum_xy_dominance += x * state.dominance;
            sum_xx += x * x;
        }
        
        let slope_valence = (n * sum_xy_valence - sum_x * sum_y_valence) / (n * sum_xx - sum_x * sum_x);
        let slope_arousal = (n * sum_xy_arousal - sum_x * sum_y_arousal) / (n * sum_xx - sum_x * sum_x);
        let slope_dominance = (n * sum_xy_dominance - sum_x * sum_y_dominance) / (n * sum_xx - sum_x * sum_x);
        
        let intercept_valence = (sum_y_valence - slope_valence * sum_x) / n;
        let intercept_arousal = (sum_y_arousal - slope_arousal * sum_x) / n;
        let intercept_dominance = (sum_y_dominance - slope_dominance * sum_x) / n;
        
        // Predict next state (n+1)
        let next_x = n;
        Some(EmotionalVector {
            valence: intercept_valence + slope_valence * next_x,
            arousal: intercept_arousal + slope_arousal * next_x,
            dominance: intercept_dominance + slope_dominance * next_x,
        })
    }
    
    /// Identify emotional patterns in the session
    pub fn identify_patterns(&self) -> Vec<PatternType> {
        if self.emotional_states.len() < 5 {
            return vec![];
        }
        
        let mut patterns = Vec::new();
        let variance = self.emotional_variance();
        
        // Check for volatility
        if variance > 0.5 {
            patterns.push(PatternType::Volatile);
        } else if variance < 0.1 {
            patterns.push(PatternType::Stable);
        }
        
        // Check for trends
        let first_avg = self.emotional_states[..self.emotional_states.len()/2]
            .iter()
            .fold(EmotionalVector::default(), |acc, state| EmotionalVector {
                valence: acc.valence + state.valence,
                arousal: acc.arousal + state.arousal,
                dominance: acc.dominance + state.dominance,
            });
        
        let second_avg = self.emotional_states[self.emotional_states.len()/2..]
            .iter()
            .fold(EmotionalVector::default(), |acc, state| EmotionalVector {
                valence: acc.valence + state.valence,
                arousal: acc.arousal + state.arousal,
                dominance: acc.dominance + state.dominance,
            });
        
        let first_count = self.emotional_states.len() / 2;
        let second_count = self.emotional_states.len() - first_count;
        
        let first_avg_valence = first_avg.valence / first_count as f32;
        let second_avg_valence = second_avg.valence / second_count as f32;
        
        if second_avg_valence > first_avg_valence + 0.2 {
            patterns.push(PatternType::Ascending);
        } else if second_avg_valence < first_avg_valence - 0.2 {
            patterns.push(PatternType::Descending);
        }
        
        patterns
    }
    
    /// Identify advanced emotional patterns
    pub fn identify_advanced_patterns(&self) -> Vec<AdvancedEmotionalPattern> {
        if self.emotional_states.len() < 5 {
            return vec![];
        }
        
        let mut patterns = Vec::new();
        let variance = self.emotional_variance();
        
        // Check for volatility patterns
        if variance > 0.5 {
            patterns.push(AdvancedEmotionalPattern {
                pattern_id: [0u8; 32],
                creator: self.creator,
                pattern_type: PatternType::Volatile,
                frequency: 1,
                last_occurrence: Clock::get().unwrap().unix_timestamp,
                associated_traits: vec!["high_variability".to_string()],
                emotional_intensity: variance,
                temporal_stability: 0.2,
                cross_session_consistency: 0.0, // Would be calculated across sessions
                predictive_confidence: 0.3, // Lower confidence for volatile patterns
                pattern_complexity: self.emotional_complexity,
                emotional_resonance: 0.6,
                creative_potential: 0.8,
            });
        } else if variance < 0.1 {
            patterns.push(AdvancedEmotionalPattern {
                pattern_id: [0u8; 32],
                creator: self.creator,
                pattern_type: PatternType::Stable,
                frequency: 1,
                last_occurrence: Clock::get().unwrap().unix_timestamp,
                associated_traits: vec!["consistent".to_string()],
                emotional_intensity: variance,
                temporal_stability: 0.9,
                cross_session_consistency: 0.0,
                predictive_confidence: 0.8, // Higher confidence for stable patterns
                pattern_complexity: self.emotional_complexity,
                emotional_resonance: 0.8,
                creative_potential: 0.4,
            });
        }
        
        // Check for trends using simple moving averages
        let first_half = &self.emotional_states[..self.emotional_states.len()/2];
        let second_half = &self.emotional_states[self.emotional_states.len()/2..];
        
        if !first_half.is_empty() && !second_half.is_empty() {
            let first_avg_valence: f32 = first_half.iter().map(|s| s.valence).sum::<f32>() / first_half.len() as f32;
            let second_avg_valence: f32 = second_half.iter().map(|s| s.valence).sum::<f32>() / second_half.len() as f32;
            
            if second_avg_valence > first_avg_valence + 0.2 {
                patterns.push(AdvancedEmotionalPattern {
                    pattern_id: [0u8; 32],
                    creator: self.creator,
                    pattern_type: PatternType::Ascending,
                    frequency: 1,
                    last_occurrence: Clock::get().unwrap().unix_timestamp,
                    associated_traits: vec!["improving_mood".to_string()],
                    emotional_intensity: (second_avg_valence - first_avg_valence).abs(),
                    temporal_stability: 0.7,
                    cross_session_consistency: 0.0,
                    predictive_confidence: 0.7,
                    pattern_complexity: self.emotional_complexity,
                    emotional_resonance: 0.7,
                    creative_potential: 0.6,
                });
            } else if second_avg_valence < first_avg_valence - 0.2 {
                patterns.push(AdvancedEmotionalPattern {
                    pattern_id: [0u8; 32],
                    creator: self.creator,
                    pattern_type: PatternType::Descending,
                    frequency: 1,
                    last_occurrence: Clock::get().unwrap().unix_timestamp,
                    associated_traits: vec!["declining_mood".to_string()],
                    emotional_intensity: (second_avg_valence - first_avg_valence).abs(),
                    temporal_stability: 0.6,
                    cross_session_consistency: 0.0,
                    predictive_confidence: 0.6,
                    pattern_complexity: self.emotional_complexity,
                    emotional_resonance: 0.5,
                    creative_potential: 0.3,
                });
            }
        }
        
        patterns
    }
    
    /// Get session emotional profile
    pub fn get_emotional_profile(&self) -> HashMap<String, f32> {
        let mut profile = HashMap::new();
        
        if self.emotional_states.is_empty() {
            return profile;
        }
        
        // Calculate averages
        let avg = self.average_emotional_state();
        profile.insert("avg_valence".to_string(), avg.valence);
        profile.insert("avg_arousal".to_string(), avg.arousal);
        profile.insert("avg_dominance".to_string(), avg.dominance);
        
        // Calculate variance
        profile.insert("emotional_variance".to_string(), self.emotional_variance());
        
        // Calculate emotional range
        let min_valence = self.emotional_states.iter().map(|s| s.valence).fold(1.0f32, f32::min);
        let max_valence = self.emotional_states.iter().map(|s| s.valence).fold(-1.0f32, f32::max);
        profile.insert("valence_range".to_string(), max_valence - min_valence);
        
        // Calculate emotional direction (slope of valence over time)
        if self.emotional_states.len() > 1 {
            let first = self.emotional_states.first().unwrap().valence;
            let last = self.emotional_states.last().unwrap().valence;
            profile.insert("emotional_direction".to_string(), last - first);
        }
        
        // Add advanced metrics
        profile.insert("emotional_complexity".to_string(), self.emotional_complexity);
        profile.insert("creativity_index".to_string(), self.creativity_index);
        profile.insert("engagement_score".to_string(), self.engagement_score);
        
        profile
    }
    
    /// Update engagement score based on interaction patterns
    pub fn update_engagement_score(&mut self, interaction_intensity: f32) {
        // Simple engagement calculation based on interaction intensity and frequency
        let frequency_factor = (self.total_interactions as f32 / 100.0).min(1.0);
        self.engagement_score = (interaction_intensity * 0.7 + frequency_factor * 0.3).min(1.0);
    }
    
    /// Initialize predictive model
    pub fn initialize_predictive_model(&mut self) {
        self.predictive_model = Some(EmotionalTrajectoryPredictor::new());
    }
    
    /// Get prediction with confidence
    pub fn get_prediction_with_confidence(&self) -> Option<(EmotionalVector, f32)> {
        if let Some(model) = &self.predictive_model {
            if let Some(prediction) = model.predict_next(1) {
                let confidence = model.prediction_confidence();
                return Some((prediction, confidence));
            }
        }
        None
    }
}

impl EmotionalTrajectory {
    /// Add compressed emotional state to trajectory
    pub fn add_state(&mut self, state: CompressedEmotionalState) {
        self.compressed_states.push(state);
    }
    
    /// Predict next emotional state in the trajectory
    pub fn predict_next_state(&mut self) -> Option<EmotionalVector> {
        if self.compressed_states.len() < 3 {
            return None;
        }
        
        // Simple prediction based on last few states
        let len = self.compressed_states.len();
        let last_state = &self.compressed_states[len-1];
        let prev_state = &self.compressed_states[len-2];
        
        let delta_v = last_state.v - prev_state.v;
        let delta_a = last_state.a as i16 - prev_state.a as i16;
        let delta_d = last_state.d as i16 - prev_state.d as i16;
        
        let predicted_v = (last_state.v as i16 + delta_v) as i8;
        let predicted_a = (last_state.a as i16 + delta_a) as u8;
        let predicted_d = (last_state.d as i16 + delta_d) as u8;
        
        // Confidence decreases with prediction distance
        self.prediction_confidence = 0.8 - (len as f32 * 0.05).min(0.7);
        
        Some(EmotionalVector {
            valence: (predicted_v as f32) / 100.0,
            arousal: (predicted_a as f32) / 100.0,
            dominance: (predicted_d as f32) / 100.0,
        })
    }
    
    /// Calculate trajectory metrics
    pub fn calculate_metrics(&mut self) {
        if self.compressed_states.is_empty() {
            return;
        }
        
        // Calculate emotional volatility
        if self.compressed_states.len() > 1 {
            let mut sum_variance = 0.0;
            for i in 1..self.compressed_states.len() {
                let prev = &self.compressed_states[i-1];
                let curr = &self.compressed_states[i];
                
                let dv = (curr.v - prev.v) as f32;
                let da = (curr.a as i16 - prev.a as i16) as f32;
                let dd = (curr.d as i16 - prev.d as i16) as f32;
                
                sum_variance += (dv * dv + da * da + dd * dd).sqrt();
            }
            
            self.metadata.emotional_volatility = sum_variance / (self.compressed_states.len() - 1) as f32;
        }
        
        // Identify dominant emotions
        let mut emotion_counts = HashMap::new();
        for state in &self.compressed_states {
            // Decompress to get emotional category
            let valence = (state.v as f32) / 100.0;
            let arousal = (state.a as f32) / 100.0;
            let dominance = (state.d as f32) / 100.0;
            
            let category = match (valence, arousal, dominance) {
                (v, a, d) if v > 0.5 && a > 0.5 => "Excited".to_string(),
                (v, a, d) if v > 0.5 && a <= 0.5 => "Happy".to_string(),
                (v, a, d) if v <= 0.5 && a > 0.5 => "Anxious".to_string(),
                _ => "Calm".to_string(),
            };
            
            *emotion_counts.entry(category).or_insert(0) += 1;
        }
        
        // Find most frequent emotions
        let mut sorted_emotions: Vec<(String, i32)> = emotion_counts.into_iter().collect();
        sorted_emotions.sort_by(|a, b| b.1.cmp(&a.1));
        
        self.metadata.dominant_emotions = sorted_emotions
            .into_iter()
            .take(3)
            .map(|(emotion, _)| emotion)
            .collect();
    }
    
    /// Enhance trajectory with advanced analytics
    pub fn enhance_with_analytics(&mut self) {
        self.calculate_metrics();
        
        // Add predictive next state if we have enough data
        if self.compressed_states.len() >= 3 {
            if let Some(predicted) = self.predict_next_state() {
                self.predicted_next_state = Some(predicted);
            }
        }
    }
    
    /// Get trajectory emotional profile
    pub fn get_profile(&self) -> HashMap<String, f32> {
        let mut profile = HashMap::new();
        
        profile.insert("volatility".to_string(), self.metadata.emotional_volatility);
        profile.insert("prediction_confidence".to_string(), self.prediction_confidence);
        profile.insert("state_count".to_string(), self.compressed_states.len() as f32);
        
        // Calculate average emotional values from compressed states
        if !self.compressed_states.is_empty() {
            let sum_v: i32 = self.compressed_states.iter().map(|s| s.v as i32).sum();
            let sum_a: u32 = self.compressed_states.iter().map(|s| s.a as u32).sum();
            let sum_d: u32 = self.compressed_states.iter().map(|s| s.d as u32).sum();
            
            profile.insert("avg_valence".to_string(), (sum_v as f32) / (self.compressed_states.len() as f32) / 100.0);
            profile.insert("avg_arousal".to_string(), (sum_a as f32) / (self.compressed_states.len() as f32) / 100.0);
            profile.insert("avg_dominance".to_string(), (sum_d as f32) / (self.compressed_states.len() as f32) / 100.0);
        }
        
        profile
    }
    
    /// Calculate emotional trajectory complexity
    pub fn calculate_trajectory_complexity(&self) -> f32 {
        if self.compressed_states.len() < 3 {
            return 0.0;
        }
        
        // Calculate the complexity based on direction changes and variance
        let mut direction_changes = 0;
        let mut total_distance = 0.0;
        
        for i in 1..self.compressed_states.len() {
            let prev = &self.compressed_states[i-1];
            let curr = &self.compressed_states[i];
            
            // Calculate distance between consecutive states
            let dv = (curr.v - prev.v) as f32;
            let da = (curr.a as i16 - prev.a as i16) as f32;
            let dd = (curr.d as i16 - prev.d as i16) as f32;
            
            let distance = (dv * dv + da * da + dd * dd).sqrt();
            total_distance += distance;
            
            // Check for significant direction changes in valence
            if i > 1 {
                let prev_prev = &self.compressed_states[i-2];
                let prev_dv = (prev.v - prev_prev.v) as f32;
                
                // If direction changed significantly
                if (dv * prev_dv) < -10.0 {
                    direction_changes += 1;
                }
            }
        }
        
        // Normalize complexity score (0-1)
        let avg_distance = total_distance / (self.compressed_states.len() - 1) as f32;
        let change_ratio = direction_changes as f32 / self.compressed_states.len() as f32;
        
        (avg_distance / 100.0 + change_ratio).min(1.0)
    }
    
    /// Identify recurring patterns in the trajectory
    pub fn identify_recurring_patterns(&self) -> Vec<PatternType> {
        if self.compressed_states.len() < 6 {
            return vec![];
        }
        
        let mut patterns = Vec::new();
        
        // Check for cyclical patterns by looking for repeated emotional states
        let mut state_frequencies = HashMap::new();
        for state in &self.compressed_states {
            let key = (state.v, state.a, state.d);
            *state_frequencies.entry(key).or_insert(0) += 1;
        }
        
        // If any state appears more than 25% of the time, it's cyclical
        let threshold = (self.compressed_states.len() as f32 * 0.25) as i32;
        if state_frequencies.values().any(|&count| count > threshold) {
            patterns.push(PatternType::Cyclical);
        }
        
        // Check for stability (low variance)
        let volatility = self.metadata.emotional_volatility;
        if volatility < 5.0 {
            patterns.push(PatternType::Stable);
        }
        
        patterns
    }
    
    /// Generate emotional trajectory summary
    pub fn generate_summary(&self) -> HashMap<String, String> {
        let mut summary = HashMap::new();
        
        // Add basic metrics
        summary.insert("state_count".to_string(), self.compressed_states.len().to_string());
        summary.insert("volatility".to_string(), format!("{:.2}", self.metadata.emotional_volatility));
        summary.insert("prediction_confidence".to_string(), format!("{:.2}", self.prediction_confidence));
        
        // Add dominant emotions
        if !self.metadata.dominant_emotions.is_empty() {
            summary.insert("dominant_emotions".to_string(), self.metadata.dominant_emotions.join(", "));
        }
        
        // Add pattern information
        let patterns = self.identify_recurring_patterns();
        if !patterns.is_empty() {
            let pattern_names: Vec<String> = patterns.iter().map(|p| {
                match p {
                    PatternType::Cyclical => "Cyclical",
                    PatternType::Ascending => "Ascending",
                    PatternType::Descending => "Descending",
                    PatternType::Stable => "Stable",
                    PatternType::Volatile => "Volatile",
                }.to_string()
            }).collect();
            summary.insert("patterns".to_string(), pattern_names.join(", "));
        }
        
        // Add complexity score
        let complexity = self.calculate_trajectory_complexity();
        summary.insert("complexity".to_string(), format!("{:.2}", complexity));
        
        summary
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
    
    #[test]
    fn test_emotional_category() {
        let happy_state = EmotionalState {
            valence: 0.8,
            arousal: 0.7,
            dominance: 0.6,
            ..Default::default()
        };
        
        let category = happy_state.get_emotional_category();
        assert_eq!(category, "Excited");
    }
    
    #[test]
    fn test_prediction() {
        let mut session = NeuroemotiveSession {
            session_id: [0u8; 32],
            creator: Pubkey::default(),
            start_time: 0,
            emotional_states: vec![
                EmotionalVector { valence: 0.1, arousal: 0.2, dominance: 0.3 },
                EmotionalVector { valence: 0.2, arousal: 0.3, dominance: 0.4 },
                EmotionalVector { valence: 0.3, arousal: 0.4, dominance: 0.5 },
            ],
            diffusion_generations: vec![],
            stream_active: false,
            total_interactions: 0,
            recognized_patterns: vec![],
            session_traits: HashMap::new(),
        };
        
        let prediction = session.predict_next_state();
        assert!(prediction.is_some());
    }
}