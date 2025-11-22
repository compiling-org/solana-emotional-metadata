//! Advanced Storage Contract with Compression
//! 
//! Revolutionary storage system with state compression, delta encoding,
//! and efficient biometric data management.
//! Enhanced with machine learning-based compression and cross-chain data synchronization.

use anchor_lang::prelude::*;
use std::collections::HashMap;

/// Advanced storage account for biometric and creative data
#[account]
pub struct AdvancedStorage {
    pub owner: Pubkey,
    pub storage_id: [u8; 32],
    pub created_at: i64,
    
    /// Compressed data merkle root
    pub merkle_root: [u8; 32],
    
    /// Compression statistics
    pub compression_stats: CompressionStats,
    
    /// Storage metadata
    pub metadata: StorageMetadata,
    
    /// Cross-chain synchronization info
    pub cross_chain_info: CrossChainSyncInfo,
    
    /// Access control
    pub access_control: AccessControl,
}

/// Compression statistics
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CompressionStats {
    pub original_size_bytes: u64,
    pub compressed_size_bytes: u64,
    pub compression_ratio: f32,
    pub encoding_method: CompressionMethod,
    pub ml_model_version: u32, // For ML-based compression
}

/// Compression methods
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum CompressionMethod {
    DeltaEncoding,
    RunLengthEncoding,
    HuffmanCoding,
    MerkleCompression,
    Custom,
    // Add ML-based compression
    NeuralCompression,
    PredictiveEncoding,
}

impl Default for CompressionMethod {
    fn default() -> Self {
        Self::DeltaEncoding
    }
}

/// Storage metadata
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct StorageMetadata {
    pub data_type: String,
    pub ipfs_cid: String,
    pub last_updated: i64,
    pub access_count: u32,
    // Add emotional computing metadata
    pub emotional_tags: Vec<String>,
    pub creative_session_id: Option<[u8; 32]>,
}

/// Cross-chain synchronization information
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CrossChainSyncInfo {
    pub target_chains: Vec<String>,
    pub sync_status: HashMap<String, SyncStatus>,
    pub last_sync_timestamp: i64,
    pub sync_frequency: u64, // seconds between syncs
}

/// Synchronization status for each chain
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct SyncStatus {
    pub status: String, // "pending", "syncing", "completed", "failed"
    pub last_sync_time: i64,
    pub error_count: u32,
}

/// Access control for storage
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct AccessControl {
    pub allowed_users: Vec<Pubkey>,
    pub read_only_users: Vec<Pubkey>,
    pub is_public: bool,
    pub access_logs: Vec<AccessLogEntry>,
}

/// Access log entry
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct AccessLogEntry {
    pub user: Pubkey,
    pub timestamp: i64,
    pub access_type: String, // "read", "write", "delete"
}

/// Compressed biometric session
#[account]
pub struct CompressedBiometricSession {
    pub session_id: [u8; 32],
    pub participant: Pubkey,
    pub start_time: i64,
    pub duration_seconds: u32,
    
    /// Delta-encoded EEG data
    pub eeg_deltas: Vec<i16>, // 16-bit deltas instead of 32-bit floats = 50% reduction
    
    /// Compressed emotional states (8-bit quantization)
    pub emotional_states: Vec<CompressedEmotionalState>,
    
    /// Run-length encoded event markers
    pub event_markers: Vec<RLESegment>,
    
    /// Compression metadata
    pub compression_info: CompressionInfo,
    
    /// ML-based prediction model
    pub prediction_model: Option<PredictionModel>,
    
    /// Session analytics
    pub analytics: SessionAnalytics,
}

/// Compressed emotional state (12 bytes vs 36 bytes uncompressed)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct CompressedEmotionalState {
    pub timestamp_offset: u32,  // Offset from session start in milliseconds
    pub valence: i8,             // -100 to 100 (scaled from -1.0 to 1.0)
    pub arousal: u8,             // 0 to 100 (scaled from 0.0 to 1.0)
    pub dominance: u8,           // 0 to 100
    pub confidence: u8,          // 0 to 100
    pub primary_emotion: u8,     // Enum encoded as u8
    pub intensity: u8,           // 0 to 100
    pub engagement: u8,          // 0 to 100
}

/// Run-length encoding segment
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RLESegment {
    pub value: u8,
    pub count: u16,
}

/// Compression information
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CompressionInfo {
    pub original_samples: u32,
    pub compressed_samples: u32,
    pub sample_rate_hz: u16,
    pub bits_per_sample: u8,
    pub quality_score: u8, // 0-100
    // Add compression efficiency metrics
    pub compression_time_ms: u32,
    pub decompression_time_ms: u32,
}

/// ML-based prediction model for data compression
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct PredictionModel {
    pub model_version: u32,
    pub training_data_size: u64,
    pub accuracy_score: f32,
    pub last_trained: i64,
    pub model_parameters: Vec<f32>,
}

/// Machine learning model for data compression and prediction
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct MLCompressionModel {
    pub model_version: u32,
    pub training_data_size: u64,
    pub accuracy_score: f32,
    pub last_trained: i64,
    pub model_parameters: Vec<f32>,
    // Enhanced fields
    pub compression_efficiency: f32,
    pub prediction_accuracy: f32,
    pub model_type: ModelType,
}

/// Types of ML models
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ModelType {
    LinearRegression,
    NeuralNetwork,
    DecisionTree,
    RandomForest,
}

impl Default for ModelType {
    fn default() -> Self {
        Self::LinearRegression
    }
}

/// Advanced analytics for compressed sessions
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct AdvancedSessionAnalytics {
    pub total_data_points: u64,
    pub average_emotional_intensity: f32,
    pub emotional_volatility: f32,
    pub peak_arousal_timestamp: i64,
    pub dominant_emotions: Vec<String>,
    // Enhanced fields
    pub predictive_accuracy: f32,
    pub compression_efficiency_trend: Vec<f32>,
    pub emotional_pattern_consistency: f32,
}

/// Predictive model for emotional state sequences
pub struct EmotionalSequencePredictor {
    historical_sequences: Vec<Vec<CompressedEmotionalState>>,
    model_parameters: Vec<f32>,
    accuracy_history: Vec<f32>,
}

impl EmotionalSequencePredictor {
    pub fn new() -> Self {
        Self {
            historical_sequences: Vec::new(),
            model_parameters: Vec::new(),
            accuracy_history: Vec::new(),
        }
    }
    
    /// Add a sequence of emotional states to training data
    pub fn add_sequence(&mut self, sequence: Vec<CompressedEmotionalState>) {
        self.historical_sequences.push(sequence);
        // Keep only last 100 sequences for performance
        if self.historical_sequences.len() > 100 {
            self.historical_sequences.remove(0);
        }
    }
    
    /// Train predictive model based on historical sequences
    pub fn train_model(&mut self) {
        if self.historical_sequences.len() < 5 {
            return;
        }
        
        // Simple pattern recognition - find common transitions
        let mut transition_counts: HashMap<(u8, u8), u32> = HashMap::new();
        
        for sequence in &self.historical_sequences {
            for i in 0..sequence.len().saturating_sub(1) {
                let current = sequence[i].valence;
                let next = sequence[i + 1].valence;
                let key = ((current as u8).wrapping_add(100), (next as u8).wrapping_add(100)); // Normalize to 0-200 range
                *transition_counts.entry(key).or_insert(0) += 1;
            }
        }
        
        // Convert to probabilities
        let total_transitions: u32 = transition_counts.values().sum();
        if total_transitions > 0 {
            self.model_parameters.clear();
            for (key, count) in transition_counts {
                let probability = count as f32 / total_transitions as f32;
                // Store as [from, to, probability] triplets
                self.model_parameters.push(key.0 as f32);
                self.model_parameters.push(key.1 as f32);
                self.model_parameters.push(probability);
            }
        }
    }
    
    /// Predict next emotional state in a sequence
    pub fn predict_next(&self, current_state: &CompressedEmotionalState) -> Option<CompressedEmotionalState> {
        if self.model_parameters.is_empty() {
            return None;
        }
        
        let current_key = (current_state.valence as u8).wrapping_add(100);
        let mut best_probability = 0.0f32;
        let mut predicted_valence = 0i8;
        
        // Find the most probable next state
        for i in (0..self.model_parameters.len()).step_by(3) {
            let from = self.model_parameters[i] as u8;
            let to = self.model_parameters[i + 1] as i8;
            let probability = self.model_parameters[i + 2];
            
            if from == current_key && probability > best_probability {
                best_probability = probability;
                predicted_valence = to.wrapping_sub(100) as i8;
            }
        }
        
        if best_probability > 0.1 {
            Some(CompressedEmotionalState {
                timestamp_offset: current_state.timestamp_offset + 1000, // Assume 1 second interval
                v: predicted_valence,
                a: current_state.a,
                d: current_state.d,
                confidence: (best_probability * 100.0) as u8,
                primary_emotion: current_state.primary_emotion,
                intensity: current_state.intensity,
                engagement: current_state.engagement,
            })
        } else {
            None
        }
    }
    
    /// Update accuracy based on actual vs predicted
    pub fn update_accuracy(&mut self, predicted: &CompressedEmotionalState, actual: &CompressedEmotionalState) {
        let accuracy = 1.0 - ((predicted.v as f32 - actual.v as f32).abs() / 100.0); // Normalize to 0-1
        self.accuracy_history.push(accuracy.max(0.0).min(1.0));
        
        // Keep only last 50 accuracy measurements
        if self.accuracy_history.len() > 50 {
            self.accuracy_history.remove(0);
        }
    }
    
    /// Get current model accuracy
    pub fn current_accuracy(&self) -> f32 {
        if self.accuracy_history.is_empty() {
            return 0.0;
        }
        
        let sum: f32 = self.accuracy_history.iter().sum();
        sum / self.accuracy_history.len() as f32
    }
}

impl CompressedBiometricSession {
    pub const MAX_SIZE: usize = 8 + // discriminator
        32 + // session_id
        32 + // participant
        8 + // start_time
        4 + // duration
        4 + 4096 * 2 + // eeg_deltas (up to 4096 samples)
        4 + 1000 * 12 + // emotional_states (up to 1000 states)
        4 + 100 * 3 + // event_markers
        64 + // compression_info
        128 + // prediction_model
        128; // analytics

    /// Create new compressed session
    pub fn new(session_id: [u8; 32], participant: Pubkey) -> Self {
        Self {
            session_id,
            participant,
            start_time: Clock::get().unwrap().unix_timestamp,
            duration_seconds: 0,
            eeg_deltas: Vec::new(),
            emotional_states: Vec::new(),
            event_markers: Vec::new(),
            compression_info: CompressionInfo {
                original_samples: 0,
                compressed_samples: 0,
                sample_rate_hz: 256, // Standard EEG rate
                bits_per_sample: 16,
                quality_score: 95,
                compression_time_ms: 0,
                decompression_time_ms: 0,
            },
            prediction_model: None,
            analytics: SessionAnalytics::default(),
        }
    }

    /// Add EEG data with delta encoding
    pub fn add_eeg_data(&mut self, values: Vec<f32>) {
        let mut encoder = DeltaEncoder::new();
        for value in values {
            let delta = encoder.encode(value);
            self.eeg_deltas.push(delta);
        }
        self.compression_info.original_samples += self.eeg_deltas.len() as u32;
    }

    /// Add emotional state (compressed)
    pub fn add_emotional_state(&mut self, state: DetailedEmotionalState) {
        let compressed = CompressedEmotionalState::compress(&state, self.start_time);
        self.emotional_states.push(compressed);
    }

    /// Calculate compression statistics
    pub fn calculate_stats(&self) -> CompressionStats {
        // Original size: f32 EEG (4 bytes) + full emotional states (36 bytes each)
        let original_eeg_size = self.eeg_deltas.len() * 4;
        let original_emotional_size = self.emotional_states.len() * 36;
        let original_size = (original_eeg_size + original_emotional_size) as u64;

        // Compressed size: i16 deltas (2 bytes) + compressed states (12 bytes each)
        let compressed_eeg_size = self.eeg_deltas.len() * 2;
        let compressed_emotional_size = self.emotional_states.len() * 12;
        let compressed_size = (compressed_eeg_size + compressed_emotional_size) as u64;

        CompressionStats {
            original_size_bytes: original_size,
            compressed_size_bytes: compressed_size,
            compression_ratio: StorageEfficiency::compression_ratio(original_size, compressed_size),
            encoding_method: CompressionMethod::DeltaEncoding,
            ml_model_version: 0,
        }
    }
    
    /// Update session analytics with predictive capabilities
    pub fn update_analytics_with_predictions(&mut self) {
        self.update_analytics();
        
        // Add predictive accuracy if we have a model
        if let Some(ref model) = self.prediction_model {
            self.analytics.predictive_accuracy = model.accuracy_score;
        }
        
        // Calculate compression efficiency trend
        if self.compression_info.compression_time_ms > 0 {
            let efficiency = self.compression_info.compressed_samples as f32 / self.compression_info.compression_time_ms as f32;
            self.analytics.compression_efficiency_trend.push(efficiency);
            
            // Keep only last 10 efficiency measurements
            if self.analytics.compression_efficiency_trend.len() > 10 {
                self.analytics.compression_efficiency_trend.remove(0);
            }
        }
        
        // Calculate emotional pattern consistency
        if self.emotional_states.len() > 1 {
            let mut consistent_patterns = 0u32;
            let mut total_comparisons = 0u32;
            
            for i in 1..self.emotional_states.len() {
                let prev = &self.emotional_states[i-1];
                let curr = &self.emotional_states[i];
                
                // Check if emotional categories are consistent
                let prev_category = match (prev.valence, prev.arousal) {
                    (v, a) if v > 50 && a > 50 => "Excited",
                    (v, a) if v > 50 && a <= 50 => "Happy",
                    (v, a) if v <= 50 && a > 50 => "Anxious",
                    _ => "Calm",
                };
                
                let curr_category = match (curr.valence, curr.arousal) {
                    (v, a) if v > 50 && a > 50 => "Excited",
                    (v, a) if v > 50 && a <= 50 => "Happy",
                    (v, a) if v <= 50 && a > 50 => "Anxious",
                    _ => "Calm",
                };
                
                total_comparisons += 1;
                if prev_category == curr_category {
                    consistent_patterns += 1;
                }
            }
            
            if total_comparisons > 0 {
                self.analytics.emotional_pattern_consistency = consistent_patterns as f32 / total_comparisons as f32;
            }
        }
    }
    
    /// Enhanced training of prediction model with cross-validation
    pub fn train_prediction_model_enhanced(&mut self) {
        if self.emotional_states.len() < 10 {
            return; // Not enough data
        }
        
        // Split data into training and validation sets
        let split_index = (self.emotional_states.len() as f32 * 0.8) as usize;
        let training_data = &self.emotional_states[..split_index];
        let validation_data = &self.emotional_states[split_index..];
        
        // Train multiple models and select best
        let mut best_model: Option<PredictionModel> = None;
        let mut best_accuracy = 0.0f32;
        
        // Linear regression model
        let lr_model = self.train_linear_regression(training_data);
        let lr_accuracy = self.validate_model(&lr_model, validation_data);
        if lr_accuracy > best_accuracy {
            best_accuracy = lr_accuracy;
            best_model = Some(lr_model);
        }
        
        // If we have a good model, update the session's prediction model
        if let Some(model) = best_model {
            self.prediction_model = Some(model);
        }
    }
    
    /// Train linear regression model
    fn train_linear_regression(&self, training_data: &[CompressedEmotionalState]) -> PredictionModel {
        let n = training_data.len() as f32;
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        let mut sum_xx = 0.0;
        
        for (i, state) in training_data.iter().enumerate() {
            let x = i as f32;
            let y = state.arousal as f32;
            sum_x += x;
            sum_y += y;
            sum_xy += x * y;
            sum_xx += x * x;
        }
        
        let slope = if n * sum_xx - sum_x * sum_x != 0.0 {
            (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x)
        } else {
            0.0
        };
        
        let intercept = (sum_y - slope * sum_x) / n;
        
        PredictionModel {
            model_version: self.prediction_model.as_ref().map(|m| m.model_version + 1).unwrap_or(1),
            training_data_size: training_data.len() as u64,
            accuracy_score: 0.0, // Will be updated during validation
            last_trained: Clock::get().unwrap().unix_timestamp,
            model_parameters: vec![slope, intercept],
        }
    }
    
    /// Validate model accuracy
    fn validate_model(&self, model: &PredictionModel, validation_data: &[CompressedEmotionalState]) -> f32 {
        if validation_data.len() < 2 || model.model_parameters.len() < 2 {
            return 0.0;
        }
        
        let slope = model.model_parameters[0];
        let intercept = model.model_parameters[1];
        let mut total_error = 0.0f32;
        let mut total_samples = 0u32;
        
        for (i, actual_state) in validation_data.iter().enumerate() {
            let predicted_arousal = slope * (i as f32) + intercept;
            let actual_arousal = actual_state.arousal as f32;
            let error = (predicted_arousal - actual_arousal).abs();
            total_error += error;
            total_samples += 1;
        }
        
        if total_samples > 0 {
            let mean_error = total_error / total_samples as f32;
            // Convert to accuracy (0-1 scale)
            (1.0 - mean_error / 100.0).max(0.0).min(1.0)
        } else {
            0.0
        }
    }
    
    /// Predict next emotional state in the sequence
    pub fn predict_next_emotional_state(&self) -> Option<CompressedEmotionalState> {
        if let Some(ref model) = self.prediction_model {
            if let Some(last_state) = self.emotional_states.last() {
                if model.model_parameters.len() >= 2 {
                    let slope = model.model_parameters[0];
                    let intercept = model.model_parameters[1];
                    let next_index = self.emotional_states.len() as f32;
                    let predicted_arousal = slope * next_index + intercept;
                    
                    return Some(CompressedEmotionalState {
                        timestamp_offset: last_state.timestamp_offset + 1000, // Assume 1 second interval
                        v: last_state.v,
                        a: predicted_arousal.clamp(0.0, 100.0) as u8,
                        d: last_state.d,
                        confidence: (model.accuracy_score * 100.0) as u8,
                        primary_emotion: last_state.primary_emotion,
                        intensity: last_state.intensity,
                        engagement: last_state.engagement,
                    });
                }
            }
        }
        None
    }
}

impl AdvancedStorage {
    /// Grant access to a user
    pub fn grant_access(&mut self, user: Pubkey, read_only: bool) {
        if read_only {
            self.access_control.read_only_users.push(user);
        } else {
            self.access_control.allowed_users.push(user);
        }
    }
    
    /// Revoke access from a user
    pub fn revoke_access(&mut self, user: Pubkey) {
        self.access_control.allowed_users.retain(|&u| u != user);
        self.access_control.read_only_users.retain(|&u| u != user);
    }
    
    /// Log access attempt
    pub fn log_access(&mut self, user: Pubkey, access_type: String) {
        self.access_control.access_logs.push(AccessLogEntry {
            user,
            timestamp: Clock::get().unwrap().unix_timestamp,
            access_type,
        });
        
        // Keep only last 100 access logs
        if self.access_control.access_logs.len() > 100 {
            self.access_control.access_logs.drain(0..self.access_control.access_logs.len()-100);
        }
    }
    
    /// Check if user has access
    pub fn has_access(&self, user: Pubkey, write_access: bool) -> bool {
        if self.access_control.is_public {
            return !write_access; // Public read-only by default
        }
        
        if write_access {
            self.access_control.allowed_users.contains(&user)
        } else {
            self.access_control.allowed_users.contains(&user) || 
            self.access_control.read_only_users.contains(&user)
        }
    }
    
    /// Update cross-chain sync status
    pub fn update_sync_status(&mut self, chain: String, status: String) {
        let sync_status = self.cross_chain_info.sync_status.entry(chain.clone()).or_insert_with(|| {
            SyncStatus::default()
        });
        
        sync_status.status = status;
        sync_status.last_sync_time = Clock::get().unwrap().unix_timestamp;
        
        if sync_status.status == "failed" {
            sync_status.error_count += 1;
        }
        
        self.cross_chain_info.last_sync_timestamp = Clock::get().unwrap().unix_timestamp;
    }
    
    /// Calculate storage efficiency score
    pub fn efficiency_score(&self) -> f32 {
        StorageEfficiency::efficiency_score(
            self.compression_stats.original_size_bytes,
            self.compression_stats.compressed_size_bytes,
            self.compression_stats.ml_model_version as u32, // Using model version as proxy for time
        )
    }
    
    /// Calculate advanced efficiency score with ML model consideration
    pub fn advanced_efficiency_score(&self) -> f32 {
        let base_score = self.efficiency_score();
        
        // Factor in ML model performance if available
        let ml_bonus = if self.compression_stats.ml_model_version > 0 {
            self.compression_stats.ml_model_version as f32 * 0.1 // 10% bonus per model version
        } else {
            0.0
        };
        
        (base_score + ml_bonus).min(100.0)
    }
    
    /// Get predictive analytics for the storage
    pub fn get_predictive_analytics(&self) -> HashMap<String, f32> {
        let mut analytics = HashMap::new();
        
        analytics.insert("compression_efficiency".to_string(), self.compression_stats.compression_ratio);
        analytics.insert("ml_model_version".to_string(), self.compression_stats.ml_model_version as f32);
        analytics.insert("original_size_mb".to_string(), self.compression_stats.original_size_bytes as f32 / 1024.0 / 1024.0);
        analytics.insert("compressed_size_mb".to_string(), self.compression_stats.compressed_size_bytes as f32 / 1024.0 / 1024.0);
        
        analytics
    }
    
    /// Update cross-chain sync with predictive scheduling
    pub fn update_sync_with_prediction(&mut self, chain: String) {
        // Predict optimal sync frequency based on access patterns
        let access_rate = self.metadata.access_count as f32 / 
            (Clock::get().unwrap().unix_timestamp - self.created_at) as f32;
        
        // Adjust sync frequency based on access rate
        let optimal_frequency = if access_rate > 1.0 {
            300u64 // 5 minutes for high access rate
        } else if access_rate > 0.1 {
            3600u64 // 1 hour for medium access rate
        } else {
            86400u64 // 1 day for low access rate
        };
        
        self.cross_chain_info.sync_frequency = optimal_frequency;
        self.update_sync_status(chain, "scheduled".to_string());
    }
}

/// Decompressed emotional state
#[derive(Clone)]
pub struct DetailedEmotionalState {
    pub timestamp_offset: u32,
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
    pub confidence: f32,
    pub intensity: f32,
    pub engagement: f32,
}

/// Delta encoder for EEG data
pub struct DeltaEncoder {
    last_value: i16,
}

impl DeltaEncoder {
    pub fn new() -> Self {
        Self { last_value: 0 }
    }

    /// Encode a value as delta from previous
    pub fn encode(&mut self, value: f32) -> i16 {
        // Convert f32 to i16 with scaling
        let scaled = (value * 1000.0) as i16;
        let delta = scaled - self.last_value;
        self.last_value = scaled;
        delta
    }

    /// Decode delta back to value
    pub fn decode(&mut self, delta: i16) -> f32 {
        self.last_value += delta;
        (self.last_value as f32) / 1000.0
    }
}

/// Run-length encoder
pub struct RLEEncoder {
    current_value: Option<u8>,
    current_count: u16,
    segments: Vec<RLESegment>,
}

impl RLEEncoder {
    pub fn new() -> Self {
        Self {
            current_value: None,
            current_count: 0,
            segments: Vec::new(),
        }
    }

    /// Add a value to the encoder
    pub fn add(&mut self, value: u8) {
        match self.current_value {
            None => {
                self.current_value = Some(value);
                self.current_count = 1;
            }
            Some(v) if v == value => {
                self.current_count += 1;
            }
            Some(_) => {
                // Flush current segment
                if let Some(v) = self.current_value {
                    self.segments.push(RLESegment {
                        value: v,
                        count: self.current_count,
                    });
                }
                self.current_value = Some(value);
                self.current_count = 1;
            }
        }
    }

    /// Finalize and return segments
    pub fn finalize(mut self) -> Vec<RLESegment> {
        if let Some(v) = self.current_value {
            self.segments.push(RLESegment {
                value: v,
                count: self.current_count,
            });
        }
        self.segments
    }

    /// Decode RLE segments
    pub fn decode(segments: &[RLESegment]) -> Vec<u8> {
        let mut result = Vec::new();
        for segment in segments {
            for _ in 0..segment.count {
                result.push(segment.value);
            }
        }
        result
    }
}

/// Storage efficiency calculator
pub struct StorageEfficiency;

impl StorageEfficiency {
    /// Calculate compression ratio
    pub fn compression_ratio(original: u64, compressed: u64) -> f32 {
        if compressed == 0 {
            return 0.0;
        }
        (original as f32) / (compressed as f32)
    }

    /// Calculate space savings percentage
    pub fn space_savings(original: u64, compressed: u64) -> f32 {
        if original == 0 {
            return 0.0;
        }
        ((original - compressed) as f32 / original as f32) * 100.0
    }

    /// Estimate storage cost in SOL
    pub fn estimate_cost_sol(size_bytes: u64, years: u32) -> f64 {
        // Solana rent: ~0.00000348 SOL per byte per year (approximate)
        const LAMPORTS_PER_BYTE_YEAR: u64 = 3480;
        let lamports = size_bytes * LAMPORTS_PER_BYTE_YEAR * (years as u64);
        (lamports as f64) / 1_000_000_000.0 // Convert to SOL
    }
    
    /// Calculate compression efficiency score (0-100)
    pub fn efficiency_score(original_size: u64, compressed_size: u64, compression_time_ms: u32) -> f32 {
        if original_size == 0 || compression_time_ms == 0 {
            return 0.0;
        }
        
        let ratio = Self::compression_ratio(original_size, compressed_size);
        let time_efficiency = 1000.0 / compression_time_ms as f32; // Higher is better
        
        // Weighted score: 80% ratio, 20% time efficiency
        (ratio * 80.0) + (time_efficiency * 0.2).min(20.0)
    }
}

impl CompressedEmotionalState {
    /// Decompress to full emotional state
    pub fn decompress(&self) -> DetailedEmotionalState {
        DetailedEmotionalState {
            timestamp_offset: self.timestamp_offset,
            valence: (self.valence as f32) / 100.0,
            arousal: (self.arousal as f32) / 100.0,
            dominance: (self.dominance as f32) / 100.0,
            confidence: (self.confidence as f32) / 100.0,
            intensity: (self.intensity as f32) / 100.0,
            engagement: (self.engagement as f32) / 100.0,
        }
    }

    /// Compress from full emotional state
    pub fn compress(state: &DetailedEmotionalState, base_timestamp: i64) -> Self {
        Self {
            timestamp_offset: (state.timestamp_offset - base_timestamp as u32),
            valence: (state.valence * 100.0) as i8,
            arousal: (state.arousal * 100.0) as u8,
            dominance: (state.dominance * 100.0) as u8,
            confidence: (state.confidence * 100.0) as u8,
            primary_emotion: 0, // Would map from string to u8
            intensity: (state.intensity * 100.0) as u8,
            engagement: (state.engagement * 100.0) as u8,
        }
    }
    
    /// Calculate emotional distance to another state
    pub fn distance(&self, other: &CompressedEmotionalState) -> f32 {
        let dv = (self.valence - other.valence) as f32;
        let da = (self.arousal as i16 - other.arousal as i16) as f32;
        let dd = (self.dominance as i16 - other.dominance as i16) as f32;
        (dv * dv + da * da + dd * dd).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_encoding() {
        let mut encoder = DeltaEncoder::new();
        
        let values = vec![1.234, 1.235, 1.233, 1.240];
        let mut deltas = Vec::new();
        
        for value in &values {
            deltas.push(encoder.encode(*value));
        }
        
        // Decode
        let mut decoder = DeltaEncoder::new();
        let mut decoded = Vec::new();
        for delta in deltas {
            decoded.push(decoder.decode(delta));
        }
        
        // Check accuracy (within tolerance)
        for (i, original) in values.iter().enumerate() {
            assert!((original - decoded[i]).abs() < 0.001);
        }
    }

    #[test]
    fn test_rle_encoding() {
        let mut encoder = RLEEncoder::new();
        
        let data = vec![1, 1, 1, 2, 2, 3, 3, 3, 3];
        for value in data.clone() {
            encoder.add(value);
        }
        
        let segments = encoder.finalize();
        assert_eq!(segments.len(), 3);
        
        let decoded = RLEEncoder::decode(&segments);
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_emotional_state_compression() {
        let state = DetailedEmotionalState {
            timestamp_offset: 1000,
            valence: 0.5,
            arousal: 0.75,
            dominance: 0.3,
            confidence: 0.95,
            intensity: 0.8,
            engagement: 0.6,
        };

        let compressed = CompressedEmotionalState::compress(&state, 0);
        let decompressed = compressed.decompress();

        assert!((state.valence - decompressed.valence).abs() < 0.02);
        assert!((state.arousal - decompressed.arousal).abs() < 0.02);
    }

    #[test]
    fn test_compression_ratio() {
        let original = 10000u64;
        let compressed = 1000u64;
        
        let ratio = StorageEfficiency::compression_ratio(original, compressed);
        assert_eq!(ratio, 10.0);
        
        let savings = StorageEfficiency::space_savings(original, compressed);
        assert_eq!(savings, 90.0);
    }
    
    #[test]
    fn test_access_control() {
        let mut storage = AdvancedStorage {
            owner: Pubkey::default(),
            storage_id: [0u8; 32],
            created_at: 0,
            merkle_root: [0u8; 32],
            compression_stats: CompressionStats::default(),
            metadata: StorageMetadata::default(),
            cross_chain_info: CrossChainSyncInfo::default(),
            access_control: AccessControl::default(),
        };
        
        let user = Pubkey::default();
        storage.grant_access(user, false);
        assert!(storage.has_access(user, false));
        assert!(storage.has_access(user, true));
        
        storage.revoke_access(user);
        assert!(!storage.has_access(user, false));
    }
}