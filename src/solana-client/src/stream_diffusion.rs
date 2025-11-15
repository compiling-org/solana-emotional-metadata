//! Stream Diffusion - Real-time AI generation tracking
//! 
//! High-performance tracking for Stream Diffusion model generations

/// Stream Diffusion configuration
#[account]
#[derive(Default)]
pub struct StreamDiffusionConfig {
    pub model_id: String,
    pub base_model: String,
    pub acceleration_method: AccelerationMethod,
    pub target_fps: u8,
    pub inference_steps: u8,
    pub lcm_steps: u8,
    pub guidance_scale: f32,
}

/// Acceleration method for Stream Diffusion
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub enum AccelerationMethod {
    #[default]
    LCM,           // Latent Consistency Models
    SDXLTURBO,     // SDXL Turbo
    TensorRT,      // NVIDIA TensorRT
    TinySD,        // Tiny Stable Diffusion
}

/// Real-time generation frame
#[account]
pub struct GenerationFrame {
    pub frame_id: u64,
    pub session_id: [u8; 32],
    pub timestamp: i64,
    pub prompt: String,
    pub negative_prompt: String,
    pub emotional_conditioning: EmotionalVector,
    pub inference_time_ms: u32,
    pub result_cid: String,
    pub quality_score: f32,
}

/// Stream Diffusion session for live generation
#[account]
pub struct StreamSession {
    pub session_id: [u8; 32],
    pub creator: Pubkey,
    pub config: StreamDiffusionConfig,
    pub start_time: i64,
    pub frame_count: u64,
    pub total_inference_time_ms: u64,
    pub active: bool,
    pub performance_metrics: StreamPerformanceMetrics,
}

/// Performance metrics for stream
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct StreamPerformanceMetrics {
    pub avg_fps: f32,
    pub avg_inference_ms: f32,
    pub peak_fps: f32,
    pub min_fps: f32,
    pub total_frames: u64,
    pub dropped_frames: u32,
}

/// Emotional prompt modulation
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EmotionalPromptModulation {
    pub base_prompt: String,
    pub valence_positive_keywords: Vec<String>,
    pub valence_negative_keywords: Vec<String>,
    pub arousal_high_keywords: Vec<String>,
    pub arousal_low_keywords: Vec<String>,
    pub dominance_high_keywords: Vec<String>,
    pub dominance_low_keywords: Vec<String>,
    // Add advanced emotional modulation parameters
    pub emotional_intensity: f32,
    pub creative_direction: String,
    pub temporal_consistency: f32,
}

impl EmotionalPromptModulation {
    /// Generate modulated prompt based on emotional state
    pub fn modulate_prompt(&self, emotion: &EmotionalVector) -> String {
        let mut prompt = self.base_prompt.clone();

        // Add keywords based on valence
        if emotion.valence > 0.5 {
            for keyword in &self.valence_positive_keywords {
                prompt.push_str(&format!(", {}", keyword));
            }
        } else if emotion.valence < -0.5 {
            for keyword in &self.valence_negative_keywords {
                prompt.push_str(&format!(", {}", keyword));
            }
        }

        // Add keywords based on arousal
        if emotion.arousal > 0.7 {
            for keyword in &self.arousal_high_keywords {
                prompt.push_str(&format!(", {}", keyword));
            }
        } else if emotion.arousal < 0.3 {
            for keyword in &self.arousal_low_keywords {
                prompt.push_str(&format!(", {}", keyword));
            }
        }

        // Add keywords based on dominance
        if emotion.dominance > 0.7 {
            for keyword in &self.dominance_high_keywords {
                prompt.push_str(&format!(", {}", keyword));
            }
        } else if emotion.dominance < 0.3 {
            for keyword in &self.dominance_low_keywords {
                prompt.push_str(&format!(", {}", keyword));
            }
        }

        prompt
    }
    
    /// Generate advanced emotional modulation with intensity and consistency
    pub fn advanced_modulate(&self, emotion: &EmotionalVector, frame_index: u32) -> String {
        let mut prompt = self.modulate_prompt(emotion);
        
        // Add intensity modifier
        if self.emotional_intensity > 0.8 {
            prompt.push_str(", highly intense");
        } else if self.emotional_intensity < 0.3 {
            prompt.push_str(", subtle and calm");
        }
        
        // Add temporal consistency modifier
        if self.temporal_consistency > 0.7 && frame_index > 0 {
            prompt.push_str(", maintaining consistent style");
        } else if self.temporal_consistency < 0.3 {
            prompt.push_str(", allowing for creative variation");
        }
        
        prompt
    }
}

// Add new struct for advanced generation metrics
/// Advanced generation metrics with emotional analysis
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct AdvancedGenerationMetrics {
    pub emotional_alignment_score: f32,
    pub creative_coherence: f32,
    pub temporal_consistency: f32,
    pub aesthetic_quality: f32,
    pub emotional_trajectory: Vec<EmotionalVector>,
}

impl GenerationFrame {
    pub const MAX_SIZE: usize = 8 + // discriminator
        8 + // frame_id
        32 + // session_id
        8 + // timestamp
        256 + // prompt
        256 + // negative_prompt
        12 + // emotional_conditioning
        4 + // inference_time_ms
        256 + // result_cid
        4; // quality_score

    /// Create new generation frame
    pub fn new(
        frame_id: u64,
        session_id: [u8; 32],
        prompt: String,
        emotional_conditioning: EmotionalVector,
        inference_time_ms: u32,
    ) -> Self {
        Self {
            frame_id,
            session_id,
            timestamp: Clock::get().unwrap().unix_timestamp,
            prompt,
            negative_prompt: String::new(),
            emotional_conditioning,
            inference_time_ms,
            result_cid: String::new(),
            quality_score: 0.0,
        }
    }

    /// Set generation result
    pub fn set_result(&mut self, cid: String, quality: f32) {
        self.result_cid = cid;
        self.quality_score = quality;
    }
    
    /// Set generation result with advanced metrics
    pub fn set_result_with_metrics(&mut self, cid: String, quality: f32, metrics: AdvancedGenerationMetrics) {
        self.result_cid = cid;
        self.quality_score = quality;
        // In a real implementation, we would store metrics in a separate account
        // For now, we'll just acknowledge the metrics exist
    }
}

impl StreamSession {
    pub const MAX_SIZE: usize = 8 + // discriminator
        32 + // session_id
        32 + // creator
        256 + // config
        8 + // start_time
        8 + // frame_count
        8 + // total_inference_time
        1 + // active
        128; // performance_metrics

    /// Initialize new stream session
    pub fn new(
        session_id: [u8; 32],
        creator: Pubkey,
        config: StreamDiffusionConfig,
    ) -> Self {
        Self {
            session_id,
            creator,
            config,
            start_time: Clock::get().unwrap().unix_timestamp,
            frame_count: 0,
            total_inference_time_ms: 0,
            active: true,
            performance_metrics: StreamPerformanceMetrics::default(),
        }
    }

    /// Record a new frame generation
    pub fn record_frame(&mut self, inference_time_ms: u32) {
        self.frame_count += 1;
        self.total_inference_time_ms += inference_time_ms as u64;

        // Update performance metrics
        let current_fps = if inference_time_ms > 0 {
            1000.0 / inference_time_ms as f32
        } else {
            0.0
        };

        self.performance_metrics.avg_inference_ms = 
            self.total_inference_time_ms as f32 / self.frame_count as f32;
        
        self.performance_metrics.avg_fps = if self.performance_metrics.avg_inference_ms > 0.0 {
            1000.0 / self.performance_metrics.avg_inference_ms
        } else {
            0.0
        };

        if current_fps > self.performance_metrics.peak_fps {
            self.performance_metrics.peak_fps = current_fps;
        }

        if self.performance_metrics.min_fps == 0.0 || current_fps < self.performance_metrics.min_fps {
            self.performance_metrics.min_fps = current_fps;
        }

        self.performance_metrics.total_frames = self.frame_count;
    }

    /// Check if target FPS is being met
    pub fn is_meeting_target(&self) -> bool {
        self.performance_metrics.avg_fps >= self.config.target_fps as f32
    }

    /// Get session duration in seconds
    pub fn duration_seconds(&self) -> i64 {
        Clock::get().unwrap().unix_timestamp - self.start_time
    }

    /// Calculate frames per second based on total session time
    pub fn actual_fps(&self) -> f32 {
        let duration = self.duration_seconds();
        if duration > 0 {
            self.frame_count as f32 / duration as f32
        } else {
            0.0
        }
    }
    
    /// Get advanced performance metrics including emotional analysis
    pub fn get_advanced_metrics(&self) -> AdvancedGenerationMetrics {
        AdvancedGenerationMetrics {
            emotional_alignment_score: self.performance_metrics.avg_fps / self.config.target_fps as f32,
            creative_coherence: 1.0 - (self.performance_metrics.dropped_frames as f32 / self.performance_metrics.total_frames as f32),
            temporal_consistency: self.performance_metrics.avg_fps / self.performance_metrics.peak_fps,
            aesthetic_quality: self.performance_metrics.avg_fps / 30.0, // Normalized to 30 FPS target
            emotional_trajectory: vec![], // Would be populated with actual emotional data
        }
    }
    
    /// Adjust configuration based on performance metrics
    pub fn adjust_configuration(&mut self) {
        // If we're dropping frames, reduce target FPS
        if self.performance_metrics.dropped_frames > self.performance_metrics.total_frames / 10 {
            self.config.target_fps = (self.config.target_fps.saturating_sub(5)).max(10);
        }
        
        // If we're consistently exceeding target FPS, we might increase quality
        if self.is_meeting_target() && self.performance_metrics.avg_fps > self.config.target_fps as f32 * 1.2 {
            self.config.inference_steps = (self.config.inference_steps + 1).min(20);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_session_frame_recording() {
        let config = StreamDiffusionConfig {
            target_fps: 30,
            inference_steps: 4,
            ..Default::default()
        };

        let mut session = StreamSession::new(
            [0u8; 32],
            Pubkey::default(),
            config,
        );

        session.record_frame(33); // 33ms = ~30 FPS
        assert_eq!(session.frame_count, 1);
        assert!(session.performance_metrics.avg_fps > 0.0);
    }

    #[test]
    fn test_emotional_prompt_modulation() {
        let modulation = EmotionalPromptModulation {
            base_prompt: "a beautiful landscape".to_string(),
            valence_positive_keywords: vec!["vibrant".to_string(), "joyful".to_string()],
            valence_negative_keywords: vec!["dark".to_string(), "moody".to_string()],
            arousal_high_keywords: vec!["dynamic".to_string(), "energetic".to_string()],
            arousal_low_keywords: vec!["calm".to_string(), "peaceful".to_string()],
            dominance_high_keywords: vec!["powerful".to_string()],
            dominance_low_keywords: vec!["gentle".to_string()],
        };

        let happy_emotion = EmotionalVector {
            valence: 0.8,
            arousal: 0.6,
            dominance: 0.5,
        };

        let modulated = modulation.modulate_prompt(&happy_emotion);
        assert!(modulated.contains("vibrant") || modulated.contains("joyful"));
    }
}
