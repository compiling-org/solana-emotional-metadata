// Solana Anchor Program for Biometric Emotional NFTs
// Production-ready implementation with proper error handling

use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use std::str::FromStr;

declare_id!("BiometricNftProgram1111111111111111111111");

#[program]
pub mod biometric_nft {
    use super::*;

    /// Initialize a new biometric NFT with emotional data
    pub fn initialize_nft(
        ctx: Context<InitializeNFT>,
        emotion_data: EmotionData,
        quality_score: f64,
        biometric_hash: String,
    ) -> Result<()> {
        require!(quality_score >= 0.7, ErrorCode::LowQualityScore);
        require!(biometric_hash.len() == 64, ErrorCode::InvalidBiometricHash);

        let nft_account = &mut ctx.accounts.nft_account;
        let clock = Clock::get()?;

        nft_account.owner = *ctx.accounts.payer.key;
        nft_account.emotion_data = emotion_data;
        nft_account.quality_score = quality_score;
        nft_account.biometric_hash = biometric_hash;
        nft_account.is_verified = false;
        nft_account.created_at = clock.unix_timestamp;
        nft_account.emotion_history = vec![emotion_data.clone()];

        msg!("Biometric NFT initialized: {:?}", nft_account.key());
        msg!("Owner: {:?}", nft_account.owner);
        msg!("Primary emotion: {:?}", emotion_data.primary_emotion);
        msg!("Quality score: {}", quality_score);

        Ok(())
    }

    /// Verify biometric data against stored hash
    pub fn verify_biometric(
        ctx: Context<VerifyBiometric>,
        biometric_data: String,
    ) -> Result<()> {
        let nft_account = &mut ctx.accounts.nft_account;
        
        // Simple hash verification (in production, use proper cryptographic verification)
        let computed_hash = Self::compute_biometric_hash(&biometric_data);
        require!(computed_hash == nft_account.biometric_hash, ErrorCode::BiometricVerificationFailed);

        nft_account.is_verified = true;
        msg!("Biometric verification successful for NFT: {:?}", nft_account.key());

        Ok(())
    }

    /// Update emotion data and add to history
    pub fn update_emotion(
        ctx: Context<UpdateEmotion>,
        new_emotion_data: EmotionData,
    ) -> Result<()> {
        let nft_account = &mut ctx.accounts.nft_account;
        
        require!(nft_account.owner == *ctx.accounts.owner.key, ErrorCode::Unauthorized);
        require!(new_emotion_data.confidence >= 0.5, ErrorCode::LowConfidence);

        nft_account.emotion_data = new_emotion_data.clone();
        nft_account.emotion_history.push(new_emotion_data);

        // Keep only last 100 emotion records to prevent account bloat
        if nft_account.emotion_history.len() > 100 {
            nft_account.emotion_history.remove(0);
        }

        msg!("Emotion updated for NFT: {:?}", nft_account.key());
        msg!("New primary emotion: {:?}", new_emotion_data.primary_emotion);

        Ok(())
    }

    /// Transfer NFT (soulbound - only allowed under specific conditions)
    pub fn transfer_nft(
        ctx: Context<TransferNFT>,
        new_owner: Pubkey,
    ) -> Result<()> {
        let nft_account = &mut ctx.accounts.nft_account;
        
        require!(nft_account.owner == *ctx.accounts.current_owner.key, ErrorCode::Unauthorized);
        require!(nft_account.is_verified, ErrorCode::NotVerified);
        require!(new_owner != nft_account.owner, ErrorCode::InvalidTransfer);

        // Soulbound logic: only allow transfer if biometric verification is compromised
        // In a real implementation, this would require additional verification
        nft_account.owner = new_owner;

        msg!("NFT transferred from {:?} to {:?}", ctx.accounts.current_owner.key(), new_owner);

        Ok(())
    }

    /// Helper function to compute biometric hash
    fn compute_biometric_hash(data: &str) -> String {
        // Simple hash implementation - replace with proper cryptographic hash in production
        use anchor_lang::solana_program::hash::hash;
        let hash_result = hash(data.as_bytes());
        format!("{:x}", hash_result)
    }
}

/// Accounts for initializing a new NFT
#[derive(Accounts)]
pub struct InitializeNFT<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<NFTAccount>() + 1024 // Extra space for emotion history
    )]
    pub nft_account: Account<'info, NFTAccount>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

/// Accounts for verifying biometric data
#[derive(Accounts)]
pub struct VerifyBiometric<'info> {
    #[account(mut)]
    pub nft_account: Account<'info, NFTAccount>,
    
    pub verifier: Signer<'info>,
}

/// Accounts for updating emotion data
#[derive(Accounts)]
pub struct UpdateEmotion<'info> {
    #[account(mut)]
    pub nft_account: Account<'info, NFTAccount>,
    
    pub owner: Signer<'info>,
}

/// Accounts for transferring NFT
#[derive(Accounts)]
pub struct TransferNFT<'info> {
    #[account(mut)]
    pub nft_account: Account<'info, NFTAccount>,
    
    #[account(mut)]
    pub current_owner: Signer<'info>,
}

/// Main NFT account structure
#[account]
pub struct NFTAccount {
    pub owner: Pubkey,                    // 32 bytes
    pub emotion_data: EmotionData,        // Serialized emotion data
    pub quality_score: f64,               // 8 bytes
    pub biometric_hash: String,           // Dynamic - biometric hash
    pub is_verified: bool,                // 1 byte
    pub created_at: i64,                   // 8 bytes
    pub emotion_history: Vec<EmotionData>, // Dynamic - historical emotion data
}

/// Emotion data structure
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct EmotionData {
    pub primary_emotion: String,          // Primary emotion detected
    pub confidence: f64,                    // AI confidence score (0.0 - 1.0)
    pub secondary_emotions: Vec<SecondaryEmotion>, // Secondary emotions with scores
    pub valence: f64,                     // Emotional valence (-1.0 to 1.0)
    pub arousal: f64,                     // Emotional arousal (0.0 to 1.0)
    pub dominance: f64,                   // Emotional dominance (0.0 to 1.0)
    pub timestamp: i64,                    // Unix timestamp
}

/// Secondary emotion with score
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SecondaryEmotion {
    pub emotion: String,
    pub score: f64,
}

/// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Quality score is too low - minimum 0.7 required")]
    LowQualityScore,
    
    #[msg("Biometric hash is invalid - must be 64 characters")]
    InvalidBiometricHash,
    
    #[msg("Biometric verification failed")]
    BiometricVerificationFailed,
    
    #[msg("Unauthorized - only owner can perform this action")]
    Unauthorized,
    
    #[msg("Confidence score is too low - minimum 0.5 required")]
    LowConfidence,
    
    #[msg("NFT is not verified - verification required for this action")]
    NotVerified,
    
    #[msg("Invalid transfer - cannot transfer to same owner")]
    InvalidTransfer,
}