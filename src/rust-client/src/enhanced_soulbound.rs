//! Enhanced soulbound token implementation with AI/ML integration

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Enhanced soulbound token with AI/ML capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedSoulboundToken {
    pub token_id: String,
    pub owner_id: String,
    pub identity_data: IdentityData,
    pub collaboration_history: Vec<CollaborationRecord>,
    pub ai_recommendations: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

/// Identity data for enhanced soulbound tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityData {
    pub biometric_profile: BiometricProfile,
    pub creative_profile: CreativeProfile,
    pub reputation_score: f32,
}

/// Biometric profile data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricProfile {
    pub fingerprint_hash: String,
    pub facial_recognition_hash: String,
    pub voice_pattern_hash: String,
    pub behavioral_patterns: Vec<BehavioralPattern>,
}

/// Creative profile data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeProfile {
    pub skill_tags: Vec<String>,
    pub experience_level: String,
    pub creative_style: String,
    pub preferred_mediums: Vec<String>,
    pub collaboration_preferences: CollaborationPreferences,
}

/// Collaboration preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationPreferences {
    pub preferred_partners: Vec<String>,
    pub skill_complementarity: f32,
    pub emotional_compatibility: f32,
    pub creative_alignment: f32,
}

/// Behavioral pattern data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralPattern {
    pub pattern_type: String,
    pub confidence: f32,
    pub frequency: f32,
    pub last_observed: DateTime<Utc>,
}

/// Collaboration record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationRecord {
    pub partner_id: String,
    pub project_id: String,
    pub success_rating: f32,
    pub timestamp: i64,
    pub skills_contributed: Vec<String>,
}

impl EnhancedSoulboundToken {
    /// Create a new enhanced soulbound token
    pub fn new(token_id: String, owner_id: String, creative_skills: Vec<String>, experience_level: String) -> Self {
        let now = Utc::now();
        
        Self {
            token_id: token_id.clone(),
            owner_id: owner_id.clone(),
            identity_data: IdentityData {
                biometric_profile: BiometricProfile {
                    fingerprint_hash: format!("fp_{}", token_id),
                    facial_recognition_hash: format!("face_{}", token_id),
                    voice_pattern_hash: format!("voice_{}", token_id),
                    behavioral_patterns: vec![],
                },
                creative_profile: CreativeProfile {
                    skill_tags: creative_skills,
                    experience_level,
                    creative_style: "experimental".to_string(),
                    preferred_mediums: vec!["digital".to_string(), "interactive".to_string()],
                    collaboration_preferences: CollaborationPreferences {
                        preferred_partners: vec![],
                        skill_complementarity: 0.7,
                        emotional_compatibility: 0.6,
                        creative_alignment: 0.8,
                    },
                },
                reputation_score: 0.5,
            },
            collaboration_history: vec![],
            ai_recommendations: vec![],
            created_at: now,
            last_updated: now,
        }
    }
    
    /// Record a collaboration
    pub fn record_collaboration(&mut self, record: CollaborationRecord) {
        self.collaboration_history.push(record);
        self.last_updated = Utc::now();
    }
    
    /// Get skill recommendations based on collaboration history
    pub fn get_skill_recommendations(&self) -> Vec<String> {
        let mut skill_counts = HashMap::new();
        
        for collaboration in &self.collaboration_history {
            if collaboration.success_rating > 3.0 {
                for skill in &collaboration.skills_contributed {
                    *skill_counts.entry(skill.clone()).or_insert(0) += 1;
                }
            }
        }
        
        let mut recommendations: Vec<String> = skill_counts
            .iter()
            .filter(|(_, &count)| count >= 2)
            .map(|(skill, _)| skill.clone())
            .collect();
            
        recommendations.extend(self.ai_recommendations.clone());
        recommendations
    }
    
    /// Add AI recommendation
    pub fn add_ai_recommendation(&mut self, recommendation: String) {
        if !self.ai_recommendations.contains(&recommendation) {
            self.ai_recommendations.push(recommendation);
            self.last_updated = Utc::now();
        }
    }
    
    /// Update reputation score
    pub fn update_reputation(&mut self, delta: f32) {
        self.identity_data.reputation_score = (self.identity_data.reputation_score + delta).clamp(0.0, 1.0);
        self.last_updated = Utc::now();
    }
}