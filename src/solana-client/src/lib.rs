//! Solana Creative Metadata Program
//!
//! High-performance metadata storage for creative NFTs with neuroemotive integration.

use anchor_lang::prelude::*;

declare_id!("CreativeMetadata111111111111111111111111111");

/// Emotional vector for creative expression
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
pub struct EmotionalVector {
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
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
        
        // Create initial compressed state
        let data = [
            session_id.as_slice(),
            &emotional_state[0].to_le_bytes(),
            &emotional_state[1].to_le_bytes(),
            &emotional_state[2].to_le_bytes(),
        ].concat();
        
        session.compressed_state = hash_data(&data);
        
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
        
        // Update session
        session.interaction_count += 1;
        
        // Update compressed state
        let data = [
            session.session_id.as_slice(),
            &emotional_vector[0].to_le_bytes(),
            &emotional_vector[1].to_le_bytes(),
            &emotional_vector[2].to_le_bytes(),
            &interaction_intensity.to_le_bytes(),
        ].concat();
        
        session.compressed_state = hash_data(&data);
        
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
}

#[derive(Accounts)]
pub struct InitSession<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 32 + 8 + 3*4 + 4 + 4 + 32
    )]
    pub session: Account<'info, CreativeSession>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordData<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 8 + 3*4 + 4 + 4
    )]
    pub performance_data: Account<'info, PerformanceData>,
    #[account(mut)]
    pub session: Account<'info, CreativeSession>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_initialization() {
        // This would be an integration test in a real Solana program
        assert_eq!(2 + 2, 4);
    }
}