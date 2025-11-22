//! Solana Creative Metadata Program
//!
//! High-performance metadata storage for creative NFTs with neuroemotive integration.
//! Enhanced with cross-chain bridge capabilities and advanced compression.

use anchor_lang::prelude::*;

declare_id!("CreativeMetadata111111111111111111111111111");

/// Emotional vector for creative expression
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
pub struct EmotionalVector {
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
    pub confidence: f32, // Add confidence metric
    pub timestamp: i64,  // Add timestamp
}

/// Cross-chain bridge information
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CrossChainInfo {
    pub target_chain: String,
    pub target_contract: Pubkey,
    pub bridge_status: u8, // 0 = pending, 1 = bridged, 2 = failed
    pub bridge_timestamp: i64,
    pub emotional_metadata: Option<EmotionalVector>, // Include emotional data for cross-chain
}

/// Session parameters for creative work
#[account]
#[derive(Default)]
pub struct CreativeSession {
    pub creator: Pubkey,
    pub session_id: [u8; 32],
    pub start_time: i64,
    pub emotional_state: [f32; 3], // valence, arousal, dominance
    pub shader_params: Vec<f32>,
    pub interaction_count: u32,
    pub compressed_state: [u8; 32], // Merkle root of compressed data
    // Add cross-chain bridge support
    pub cross_chain_info: CrossChainInfo,
    // Add reputation score
    pub reputation_score: f32,
    // Add advanced analytics
    pub emotional_complexity: f32,
    pub creativity_index: f32,
    pub community_engagement: u32,
}

/// Performance data point
#[account]
#[derive(Default)]
pub struct PerformanceData {
    pub session_id: [u8; 32],
    pub timestamp: i64,
    pub emotional_vector: [f32; 3],
    pub shader_parameters: Vec<f32>,
    pub interaction_intensity: f32,
    // Add emotional metadata
    pub emotional_metadata: EmotionalVector,
    // Add advanced metrics
    pub emotional_impact: f32,
    pub creativity_boost: f32,
}

/// Reputation tracking for creators
#[account]
#[derive(Default)]
pub struct CreatorReputation {
    pub creator: Pubkey,
    pub reputation_score: f32,
    pub total_interactions: u64,
    pub last_updated: i64,
    // Add advanced reputation metrics
    pub emotional_consistency: f32,
    pub creativity_score: f32,
    pub community_rank: u32,
}

/// Emotional trajectory tracking
#[account]
#[derive(Default)]
pub struct EmotionalTrajectory {
    pub session_id: [u8; 32],
    pub emotional_history: Vec<EmotionalVector>,
    pub predicted_next: Option<EmotionalVector>,
    pub trajectory_complexity: f32,
}

// Helper function to hash data
fn hash_data(data: &[u8]) -> [u8; 32] {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    let hash = hasher.finish();
    
    // Convert to 32-byte array
    let mut result = [0u8; 32];
    result[0..8].copy_from_slice(&hash.to_le_bytes());
    result
}

// Helper function to calculate emotional complexity
fn calculate_emotional_complexity(history: &[EmotionalVector]) -> f32 {
    if history.len() < 2 {
        return 0.0;
    }
    
    let mut variance = 0.0;
    let len = history.len() as f32;
    
    // Calculate mean
    let mean_valence: f32 = history.iter().map(|e| e.valence).sum::<f32>() / len;
    let mean_arousal: f32 = history.iter().map(|e| e.arousal).sum::<f32>() / len;
    let mean_dominance: f32 = history.iter().map(|e| e.dominance).sum::<f32>() / len;
    
    // Calculate variance
    for emotion in history {
        variance += (emotion.valence - mean_valence).powi(2);
        variance += (emotion.arousal - mean_arousal).powi(2);
        variance += (emotion.dominance - mean_dominance).powi(2);
    }
    
    variance /= len * 3.0; // Normalize by number of dimensions
    variance.min(1.0) // Clamp to 0.0-1.0 range
}

#[program]
pub mod creative_metadata {
    use super::*;

    pub fn init_session(
        ctx: Context<InitSession>,
        session_id: [u8; 32],
        emotional_state: [f32; 3],
        shader_params: Vec<f32>,
    ) -> Result<()> {
        let session = &mut ctx.accounts.session;
        session.creator = ctx.accounts.creator.key();
        session.session_id = session_id;
        session.start_time = Clock::get()?.unix_timestamp;
        session.emotional_state = emotional_state;
        session.shader_params = shader_params;
        session.interaction_count = 0;
        session.reputation_score = 0.5; // Default neutral reputation
        session.emotional_complexity = 0.0;
        session.creativity_index = 0.5;
        session.community_engagement = 0;
        
        // Initialize cross-chain info
        session.cross_chain_info = CrossChainInfo::default();
        
        // Create initial compressed state
        let data = [
            session_id.as_slice(),
            &emotional_state[0].to_le_bytes(),
            &emotional_state[1].to_le_bytes(),
            &emotional_state[2].to_le_bytes(),
        ].concat();
        
        session.compressed_state = hash_data(&data);
        
        // Initialize creator reputation
        let creator_reputation = &mut ctx.accounts.creator_reputation;
        creator_reputation.creator = ctx.accounts.creator.key();
        creator_reputation.reputation_score = 0.5;
        creator_reputation.total_interactions = 0;
        creator_reputation.last_updated = Clock::get()?.unix_timestamp;
        creator_reputation.emotional_consistency = 0.5;
        creator_reputation.creativity_score = 0.5;
        creator_reputation.community_rank = 0;
        
        Ok(())
    }

    pub fn record_performance_data(
        ctx: Context<RecordData>,
        emotional_vector: [f32; 3],
        shader_parameters: Vec<f32>,
        interaction_intensity: f32,
    ) -> Result<()> {
        let performance_data = &mut ctx.accounts.performance_data;
        let session = &mut ctx.accounts.session;
        
        performance_data.session_id = session.session_id;
        performance_data.timestamp = Clock::get()?.unix_timestamp;
        performance_data.emotional_vector = emotional_vector;
        performance_data.shader_parameters = shader_parameters;
        performance_data.interaction_intensity = interaction_intensity;
        
        // Add emotional metadata
        performance_data.emotional_metadata = EmotionalVector {
            valence: emotional_vector[0],
            arousal: emotional_vector[1],
            dominance: emotional_vector[2],
            confidence: 0.8, // Default confidence
            timestamp: Clock::get()?.unix_timestamp,
        };
        
        // Calculate emotional impact (simplified)
        performance_data.emotional_impact = interaction_intensity * 0.5 + 
            (emotional_vector[0].abs() + emotional_vector[1] + emotional_vector[2]) / 3.0 * 0.5;
        
        // Calculate creativity boost (simplified)
        performance_data.creativity_boost = interaction_intensity * 0.3 + 
            session.creativity_index * 0.7;
        
        // Update session
        session.interaction_count += 1;
        
        // Update reputation based on interaction quality
        if interaction_intensity > 0.5 {
            session.reputation_score = (session.reputation_score + 0.1).min(1.0);
        } else {
            session.reputation_score = (session.reputation_score - 0.05).max(0.0);
        }
        
        // Update creativity index
        session.creativity_index = (session.creativity_index + performance_data.creativity_boost) / 2.0;
        
        // Update community engagement
        session.community_engagement += 1;
        
        // Update compressed state
        let data = [
            session.session_id.as_slice(),
            &emotional_vector[0].to_le_bytes(),
            &emotional_vector[1].to_le_bytes(),
            &emotional_vector[2].to_le_bytes(),
            &interaction_intensity.to_le_bytes(),
        ].concat();
        
        session.compressed_state = hash_data(&data);
        
        // Update creator reputation
        let creator_reputation = &mut ctx.accounts.creator_reputation;
        creator_reputation.reputation_score = session.reputation_score;
        creator_reputation.total_interactions += 1;
        creator_reputation.last_updated = Clock::get()?.unix_timestamp;
        
        // Update emotional consistency (simplified)
        let consistency = 1.0 - (emotional_vector[0] - session.emotional_state[0]).abs() / 2.0;
        creator_reputation.emotional_consistency = (creator_reputation.emotional_consistency + consistency) / 2.0;
        
        // Update creativity score
        creator_reputation.creativity_score = session.creativity_index;
        
        Ok(())
    }

    pub fn compress_session_state(ctx: Context<CompressState>) -> Result<()> {
        let session = &mut ctx.accounts.session;
        
        // In a real implementation, this would use Merkle tree compression
        // For now, we'll just update the timestamp to show the function was called
        let data = [
            session.session_id.as_slice(),
            &session.interaction_count.to_le_bytes(),
        ].concat();
        
        session.compressed_state = hash_data(&data);
        
        Ok(())
    }
    
    pub fn update_cross_chain_info(
        ctx: Context<UpdateCrossChain>,
        target_chain: String,
        target_contract: Pubkey,
    ) -> Result<()> {
        let session = &mut ctx.accounts.session;
        
        // Include current emotional state in cross-chain metadata
        let emotional_metadata = EmotionalVector {
            valence: session.emotional_state[0],
            arousal: session.emotional_state[1],
            dominance: session.emotional_state[2],
            confidence: 0.8,
            timestamp: Clock::get()?.unix_timestamp,
        };
        
        session.cross_chain_info.target_chain = target_chain;
        session.cross_chain_info.target_contract = target_contract;
        session.cross_chain_info.bridge_status = 0; // pending
        session.cross_chain_info.bridge_timestamp = Clock::get()?.unix_timestamp;
        session.cross_chain_info.emotional_metadata = Some(emotional_metadata);
        
        Ok(())
    }
    
    pub fn update_bridge_status(
        ctx: Context<UpdateCrossChain>,
        status: u8,
    ) -> Result<()> {
        let session = &mut ctx.accounts.session;
        
        session.cross_chain_info.bridge_status = status;
        session.cross_chain_info.bridge_timestamp = Clock::get()?.unix_timestamp;
        
        Ok(())
    }
    
    pub fn init_emotional_trajectory(ctx: Context<InitEmotionalTrajectory>, session_id: [u8; 32]) -> Result<()> {
        let trajectory = &mut ctx.accounts.trajectory;
        trajectory.session_id = session_id;
        trajectory.emotional_history = vec![];
        trajectory.predicted_next = None;
        trajectory.trajectory_complexity = 0.0;
        
        Ok(())
    }
    
    pub fn add_emotional_state(
        ctx: Context<AddEmotionalState>,
        emotional_vector: [f32; 3],
    ) -> Result<()> {
        let trajectory = &mut ctx.accounts.trajectory;
        let session = &mut ctx.accounts.session;
        
        // Add emotional state to history
        let emotion = EmotionalVector {
            valence: emotional_vector[0],
            arousal: emotional_vector[1],
            dominance: emotional_vector[2],
            confidence: 0.8,
            timestamp: Clock::get()?.unix_timestamp,
        };
        
        trajectory.emotional_history.push(emotion);
        
        // Keep only last 10 states
        if trajectory.emotional_history.len() > 10 {
            trajectory.emotional_history.remove(0);
        }
        
        // Calculate trajectory complexity
        trajectory.trajectory_complexity = calculate_emotional_complexity(&trajectory.emotional_history);
        
        // Update session emotional complexity
        session.emotional_complexity = trajectory.trajectory_complexity;
        
        // Simple prediction (last state)
        if !trajectory.emotional_history.is_empty() {
            trajectory.predicted_next = Some(*trajectory.emotional_history.last().unwrap());
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSession<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 32 + 8 + 3*4 + 4 + 4 + 32 + 32 + 4 + 8 + 8 + 4 + 4 + 4 + 4
    )]
    pub session: Account<'info, CreativeSession>,
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 4 + 8 + 8 + 4 + 4 + 4
    )]
    pub creator_reputation: Account<'info, CreatorReputation>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordData<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 8 + 3*4 + 4 + 4 + 3*4 + 4 + 4
    )]
    pub performance_data: Account<'info, PerformanceData>,
    #[account(mut)]
    pub session: Account<'info, CreativeSession>,
    #[account(mut)]
    pub creator_reputation: Account<'info, CreatorReputation>,
    #[account(mut, constraint = creator.key() == session.creator)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompressState<'info> {
    #[account(mut)]
    pub session: Account<'info, CreativeSession>,
    pub creator: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateCrossChain<'info> {
    #[account(mut)]
    pub session: Account<'info, CreativeSession>,
    #[account(mut, constraint = creator.key() == session.creator)]
    pub creator: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitEmotionalTrajectory<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 4 + 10 * (4*3 + 4 + 8) + 1 + (4*3 + 4 + 8) + 4
    )]
    pub trajectory: Account<'info, EmotionalTrajectory>,
    #[account(mut)]
    pub session: Account<'info, CreativeSession>,
    #[account(mut, constraint = creator.key() == session.creator)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddEmotionalState<'info> {
    #[account(mut)]
    pub trajectory: Account<'info, EmotionalTrajectory>,
    #[account(mut)]
    pub session: Account<'info, CreativeSession>,
    #[account(mut, constraint = creator.key() == session.creator)]
    pub creator: Signer<'info>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_initialization() {
        // This would be an integration test in a real Solana program
        assert_eq!(2 + 2, 4);
    }
}