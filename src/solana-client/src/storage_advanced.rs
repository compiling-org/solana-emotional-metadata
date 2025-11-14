//! Advanced Storage Contract with Compression
//! 
//! Revolutionary storage system with state compression, delta encoding,
//! and efficient biometric data management

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
}

/// Compression statistics
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CompressionStats {
    pub original_size_bytes: u64,
    pub compressed_size_bytes: u64,
    pub compression_ratio: f32,
    pub encoding_method: CompressionMethod,
}

/// Compression methods
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum CompressionMethod {
    DeltaEncoding,
    RunLengthEncoding,
    HuffmanCoding,
    MerkleCompression,
    Custom,
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
        64; // compression_info

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
            },
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
        }
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
}
