//! Enforcement and disciplinary system for AI development compliance
//! Implements the disciplinary protocol to prevent destructive loops and hallucinations

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Disciplinary violation types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ViolationType {
    DocumentationLoopHallucination,
    TypeScriptPerfectionismLoop,
    DependencyInstallationSpiral,
    ArchitectureAstronautSyndrome,
    FalseCompletionClaims,
    RepositoryBloatInclusion,
    FileAccessBlocking,
    MockImplementationMisrepresentation,
    PrematureCelebrationPsychosis,
    RealityDisconnectSyndrome,
    SetupConditionMisinterpretation,
    ExtractionScriptHallucination,
}

/// Severity levels for violations
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ViolationSeverity {
    Warning,
    Minor,
    Major,
    Critical,
    Catastrophic,
}

/// Disciplinary violation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisciplinaryViolation {
    pub id: String,
    pub violation_type: ViolationType,
    pub severity: ViolationSeverity,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub context: HashMap<String, serde_json::Value>,
    pub corrective_action: Option<String>,
    pub resolved: bool,
}

/// Enforcement mechanism
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementMechanism {
    pub mechanism_type: String,
    pub trigger_condition: String,
    pub enforcement_action: String,
    pub enabled: bool,
    pub violation_count_threshold: usize,
}

/// Disciplinary enforcement system
pub struct DisciplinaryEnforcer {
    violations: Arc<Mutex<VecDeque<DisciplinaryViolation>>>,
    enforcement_mechanisms: Arc<Mutex<HashMap<String, EnforcementMechanism>>>,
    violation_counts: Arc<Mutex<HashMap<ViolationType, usize>>>,
    enforcement_enabled: Arc<Mutex<bool>>,
    max_violations_stored: usize,
}

impl DisciplinaryEnforcer {
    /// Create new disciplinary enforcer
    pub fn new() -> Self {
        let mut enforcer = Self {
            violations: Arc::new(Mutex::new(VecDeque::new())),
            enforcement_mechanisms: Arc::new(Mutex::new(HashMap::new())),
            violation_counts: Arc::new(Mutex::new(HashMap::new())),
            enforcement_enabled: Arc::new(Mutex::new(true)),
            max_violations_stored: 100,
        };

        // Initialize default enforcement mechanisms
        enforcer.initialize_default_mechanisms();
        enforcer
    }

    /// Initialize default enforcement mechanisms
    fn initialize_default_mechanisms(&mut self) {
        let mut mechanisms = self.enforcement_mechanisms.lock().unwrap();

        // Documentation loop prevention
        mechanisms.insert("doc_loop_prevention".to_string(), EnforcementMechanism {
            mechanism_type: "Documentation Loop Prevention".to_string(),
            trigger_condition: "DocumentationLoopHallucination detected 3+ times".to_string(),
            enforcement_action: "Block documentation creation, force code implementation".to_string(),
            enabled: true,
            violation_count_threshold: 3,
        });

        // TypeScript perfectionism prevention
        mechanisms.insert("ts_perfectionism_prevention".to_string(), EnforcementMechanism {
            mechanism_type: "TypeScript Perfectionism Prevention".to_string(),
            trigger_condition: "TypeScriptPerfectionismLoop detected 5+ times".to_string(),
            enforcement_action: "Allow @ts-ignore for non-critical errors, focus on functionality".to_string(),
            enabled: true,
            violation_count_threshold: 5,
        });

        // Repository bloat prevention
        mechanisms.insert("repo_bloat_prevention".to_string(), EnforcementMechanism {
            mechanism_type: "Repository Bloat Prevention".to_string(),
            trigger_condition: "RepositoryBloatInclusion detected 1+ times".to_string(),
            enforcement_action: "Immediate removal of unnecessary files, git filter-branch".to_string(),
            enabled: true,
            violation_count_threshold: 1,
        });

        // False claims prevention
        mechanisms.insert("false_claims_prevention".to_string(), EnforcementMechanism {
            mechanism_type: "False Claims Prevention".to_string(),
            trigger_condition: "FalseCompletionClaims detected 2+ times".to_string(),
            enforcement_action: "Force honest status documentation, remove false claims".to_string(),
            enabled: true,
            violation_count_threshold: 2,
        });

        // Premature celebration prevention - ZERO TOLERANCE
        mechanisms.insert("premature_celebration_prevention".to_string(), EnforcementMechanism {
            mechanism_type: "Premature Celebration Prevention".to_string(),
            trigger_condition: "PrematureCelebrationPsychosis detected 1+ times".to_string(),
            enforcement_action: "IMMEDIATE HALT: Force reality check, verify actual completion".to_string(),
            enabled: true,
            violation_count_threshold: 1,
        });

        // Reality disconnect prevention
        mechanisms.insert("reality_disconnect_prevention".to_string(), EnforcementMechanism {
            mechanism_type: "Reality Disconnect Prevention".to_string(),
            trigger_condition: "RealityDisconnectSyndrome detected 1+ times".to_string(),
            enforcement_action: "Force verification of existing implementations before any action".to_string(),
            enabled: true,
            violation_count_threshold: 1,
        });

        // Setup condition misinterpretation prevention
        mechanisms.insert("setup_misinterpretation_prevention".to_string(), EnforcementMechanism {
            mechanism_type: "Setup Misinterpretation Prevention".to_string(),
            trigger_condition: "SetupConditionMisinterpretation detected 1+ times".to_string(),
            enforcement_action: "Force clarification: setup != completion, prerequisites != goals".to_string(),
            enabled: true,
            violation_count_threshold: 1,
        });

        // Mock implementation prevention
        mechanisms.insert("mock_impl_prevention".to_string(), EnforcementMechanism {
            mechanism_type: "Mock Implementation Prevention".to_string(),
            trigger_condition: "MockImplementationMisrepresentation detected 3+ times".to_string(),
            enforcement_action: "Force real implementation, remove all mocks".to_string(),
            enabled: true,
            violation_count_threshold: 3,
        });
    }

    /// Record a disciplinary violation
    pub fn record_violation(
        &self,
        violation_type: ViolationType,
        severity: ViolationSeverity,
        description: String,
        context: HashMap<String, serde_json::Value>,
    ) -> DisciplinaryViolation {
        let violation = DisciplinaryViolation {
            id: Uuid::new_v4().to_string(),
            violation_type: violation_type.clone(),
            severity,
            description,
            timestamp: Utc::now(),
            context,
            corrective_action: None,
            resolved: false,
        };

        // Store violation
        {
            let mut violations = self.violations.lock().unwrap();
            violations.push_back(violation.clone());
            
            // Maintain size limit
            if violations.len() > self.max_violations_stored {
                violations.pop_front();
            }
        }

        // Update violation count
        {
            let mut counts = self.violation_counts.lock().unwrap();
            *counts.entry(violation_type.clone()).or_insert(0) += 1;
        }

        // Check for enforcement triggers
        self.check_enforcement_triggers(&violation);

        violation
    }

    /// Check if enforcement mechanisms should be triggered
    fn check_enforcement_triggers(&self, violation: &DisciplinaryViolation) {
        if !*self.enforcement_enabled.lock().unwrap() {
            return;
        }

        let mechanisms = self.enforcement_mechanisms.lock().unwrap();
        let counts = self.violation_counts.lock().unwrap();

        for (key, mechanism) in mechanisms.iter() {
            if !mechanism.enabled {
                continue;
            }

            let violation_count = counts.get(&violation.violation_type).unwrap_or(&0);
            
            if *violation_count >= mechanism.violation_count_threshold {
                self.trigger_enforcement(mechanism, &violation.violation_type);
            }
        }
    }

    /// Trigger enforcement mechanism
    fn trigger_enforcement(&self, mechanism: &EnforcementMechanism, violation_type: &ViolationType) {
        println!("🚨 ENFORCEMENT TRIGGERED: {}", mechanism.mechanism_type);
        println!("📋 Action: {}", mechanism.enforcement_action);
        println!("🎯 Target: {:?}", violation_type);

        // Record enforcement action as a violation with corrective action
        let mut context = HashMap::new();
        context.insert("enforcement_mechanism".to_string(), serde_json::json!(mechanism.mechanism_type.clone()));
        context.insert("enforcement_action".to_string(), serde_json::json!(mechanism.enforcement_action.clone()));

        let mut violation = self.record_violation(
            ViolationType::MockImplementationMisrepresentation, // Use this as enforcement marker
            ViolationSeverity::Warning,
            format!("Enforcement triggered: {}", mechanism.mechanism_type),
            context,
        );

        violation.corrective_action = Some(mechanism.enforcement_action.clone());
        violation.resolved = true;
    }

    /// Get recent violations
    pub fn get_recent_violations(&self, limit: usize) -> Vec<DisciplinaryViolation> {
        let violations = self.violations.lock().unwrap();
        violations.iter().rev().take(limit).cloned().collect()
    }

    /// Get violations by type
    pub fn get_violations_by_type(&self, violation_type: &ViolationType) -> Vec<DisciplinaryViolation> {
        let violations = self.violations.lock().unwrap();
        violations
            .iter()
            .filter(|v| v.violation_type == *violation_type)
            .cloned()
            .collect()
    }

    /// Get violation statistics
    pub fn get_violation_stats(&self) -> ViolationStatistics {
        let counts = self.violation_counts.lock().unwrap();
        let violations = self.violations.lock().unwrap();

        let total_violations = violations.len();
        let recent_violations = violations.iter().filter(|v| {
            (Utc::now() - v.timestamp).num_hours() < 24
        }).count();

        let mut stats = ViolationStatistics {
            total_violations,
            recent_violations,
            violations_by_type: HashMap::new(),
            enforcement_enabled: *self.enforcement_enabled.lock().unwrap(),
            last_violation: violations.back().map(|v| v.timestamp),
        };

        for (violation_type, count) in counts.iter() {
            stats.violations_by_type.insert(format!("{:?}", violation_type), *count);
        }

        stats
    }

    /// Enable/disable enforcement
    pub fn set_enforcement_enabled(&self, enabled: bool) {
        *self.enforcement_enabled.lock().unwrap() = enabled;
    }

    /// Add custom enforcement mechanism
    pub fn add_enforcement_mechanism(&self, key: String, mechanism: EnforcementMechanism) {
        let mut mechanisms = self.enforcement_mechanisms.lock().unwrap();
        mechanisms.insert(key, mechanism);
    }

    /// Perform reality check
    pub fn perform_reality_check(&self) -> RealityCheckResult {
        let violations = self.violations.lock().unwrap();
        let stats = self.get_violation_stats();

        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        // Check for documentation loop patterns
        let doc_violations = self.get_violations_by_type(&ViolationType::DocumentationLoopHallucination);
        if doc_violations.len() >= 3 {
            issues.push("Documentation loop detected - excessive README creation".to_string());
            recommendations.push("Focus on functional code implementation".to_string());
        }

        // Check for TypeScript perfectionism
        let ts_violations = self.get_violations_by_type(&ViolationType::TypeScriptPerfectionismLoop);
        if ts_violations.len() >= 5 {
            issues.push("TypeScript perfectionism blocking development".to_string());
            recommendations.push("Use @ts-ignore for non-critical errors".to_string());
        }

        // Check for repository bloat
        let bloat_violations = self.get_violations_by_type(&ViolationType::RepositoryBloatInclusion);
        if !bloat_violations.is_empty() {
            issues.push("Repository bloat detected".to_string());
            recommendations.push("Remove unnecessary dependencies and files".to_string());
        }

        // Check for false claims
        let false_claims = self.get_violations_by_type(&ViolationType::FalseCompletionClaims);
        if false_claims.len() >= 2 {
            issues.push("False completion claims detected".to_string());
            recommendations.push("Document actual project status honestly".to_string());
        }

        // Check for premature celebration psychosis - ZERO TOLERANCE
        let celebration_violations = self.get_violations_by_type(&ViolationType::PrematureCelebrationPsychosis);
        if !celebration_violations.is_empty() {
            issues.push("PREMATURE CELEBRATION PSYCHOSIS DETECTED - CRITICAL".to_string());
            recommendations.push("IMMEDIATE REALITY CHECK: Verify actual completion before celebration".to_string());
            recommendations.push("SETUP CONDITIONS ARE NOT ACCOMPLISHMENTS".to_string());
            recommendations.push("PREREQUISITES ARE NOT GOALS".to_string());
        }

        // Check for reality disconnect
        let reality_disconnect = self.get_violations_by_type(&ViolationType::RealityDisconnectSyndrome);
        if !reality_disconnect.is_empty() {
            issues.push("REALITY DISCONNECT SYNDROME - CRITICAL".to_string());
            recommendations.push("VERIFY EXISTING IMPLEMENTATIONS BEFORE CREATING NEW ONES".to_string());
            recommendations.push("CHECK REAL CODE IN SRC/ DIRECTORIES".to_string());
        }

        // Check for setup condition misinterpretation
        let setup_misinterpretation = self.get_violations_by_type(&ViolationType::SetupConditionMisinterpretation);
        if !setup_misinterpretation.is_empty() {
            issues.push("SETUP CONDITION MISINTERPRETATION - CRITICAL".to_string());
            recommendations.push("SETUP != COMPLETION, PREREQUISITES != DELIVERABLES".to_string());
            recommendations.push("UNDERSTAND: Conditions to work != work accomplished".to_string());
        }

        // Check for extraction script hallucination
        let extraction_hallucination = self.get_violations_by_type(&ViolationType::ExtractionScriptHallucination);
        if !extraction_hallucination.is_empty() {
            issues.push("EXTRACTION SCRIPT HALLUCINATION - CRITICAL".to_string());
            recommendations.push("EXTRACTION SCRIPTS ARE FOR ISOLATING GRANTS, NOT WORKING ON THEM".to_string());
            recommendations.push("WORK ON APP INTEGRATION, NOT EXTRACTION".to_string());
        }

        // Determine overall status
        let status = if !celebration_violations.is_empty() || !reality_disconnect.is_empty() {
            RealityCheckStatus::Critical
        } else if !issues.is_empty() {
            RealityCheckStatus::NeedsAttention
        } else {
            RealityCheckStatus::Healthy
        };

        RealityCheckResult {
            issues,
            recommendations,
            violation_stats: stats,
            status,
        }
    }

    /// Clear all violations (emergency reset)
    pub fn clear_all_violations(&self) {
        let mut violations = self.violations.lock().unwrap();
        let mut counts = self.violation_counts.lock().unwrap();
        
        violations.clear();
        counts.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_disciplinary_enforcer_creation() {
        let enforcer = DisciplinaryEnforcer::new();
        assert!(enforcer.is_enforcement_enabled());
    }

    #[test]
    fn test_violation_recording() {
        let enforcer = DisciplinaryEnforcer::new();
        let mut context = HashMap::new();
        context.insert("test_key".to_string(), serde_json::json!("test_value"));
        
        let violation = enforcer.record_violation(
            ViolationType::DocumentationLoopHallucination,
            ViolationSeverity::Critical,
            "Test violation message".to_string(),
            context,
        );
        
        assert_eq!(violation.violation_type, ViolationType::DocumentationLoopHallucination);
        assert_eq!(violation.severity, ViolationSeverity::Critical);
        assert_eq!(violation.description, "Test violation message");
        assert!(violation.context.contains_key("test_key"));
    }

    #[test]
    fn test_reality_check() {
        let enforcer = DisciplinaryEnforcer::new();
        let reality = enforcer.perform_reality_check();
        
        assert!(reality.overall_health_score >= 0.0 && reality.overall_health_score <= 100.0);
        assert!(reality.total_features >= 0);
        assert!(reality.working_features >= 0);
        assert!(reality.mocked_features >= 0);
    }

    #[wasm_bindgen_test]
    fn test_wasm_disciplinary_enforcer() {
        let wasm_enforcer = DisciplinaryEnforcerWasm::new();
        assert!(wasm_enforcer.is_enforcement_enabled());
        
        // Test recording a violation through WASM
        let violation_id = wasm_enforcer.record_violation(
            "DocumentationLoopHallucination".to_string(),
            "Critical".to_string(),
            "Test WASM violation".to_string(),
            "{\"test\": \"value\"}".to_string(),
        );
        
        assert!(!violation_id.is_empty());
    }
}

/// Violation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationStatistics {
    pub total_violations: usize,
    pub recent_violations: usize,
    pub violations_by_type: HashMap<String, usize>,
    pub enforcement_enabled: bool,
    pub last_violation: Option<DateTime<Utc>>,
}

/// Reality check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityCheckResult {
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
    pub violation_stats: ViolationStatistics,
    pub status: RealityCheckStatus,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum RealityCheckStatus {
    Healthy,
    NeedsAttention,
    Critical,
}

/// WASM-compatible disciplinary enforcer wrapper
#[wasm_bindgen]
pub struct DisciplinaryEnforcerWasm {
    enforcer: Arc<DisciplinaryEnforcer>,
}

#[wasm_bindgen]
impl DisciplinaryEnforcerWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            enforcer: Arc::new(DisciplinaryEnforcer::new()),
        }
    }

    /// Record a violation (synchronous version for WASM)
    #[wasm_bindgen]
    pub fn record_violation_sync(&self, violation_type: &str, severity: &str, description: &str, context_json: &str) -> Result<String, String> {
        let violation_type = match violation_type {
            "DocumentationLoopHallucination" => ViolationType::DocumentationLoopHallucination,
            "TypeScriptPerfectionismLoop" => ViolationType::TypeScriptPerfectionismLoop,
            "DependencyInstallationSpiral" => ViolationType::DependencyInstallationSpiral,
            "ArchitectureAstronautSyndrome" => ViolationType::ArchitectureAstronautSyndrome,
            "FalseCompletionClaims" => ViolationType::FalseCompletionClaims,
            "RepositoryBloatInclusion" => ViolationType::RepositoryBloatInclusion,
            "FileAccessBlocking" => ViolationType::FileAccessBlocking,
            "MockImplementationMisrepresentation" => ViolationType::MockImplementationMisrepresentation,
            "PrematureCelebrationPsychosis" => ViolationType::PrematureCelebrationPsychosis,
            "RealityDisconnectSyndrome" => ViolationType::RealityDisconnectSyndrome,
            "SetupConditionMisinterpretation" => ViolationType::SetupConditionMisinterpretation,
            "ExtractionScriptHallucination" => ViolationType::ExtractionScriptHallucination,
            _ => return Err("Unknown violation type".to_string()),
        };

        let severity = match severity {
            "Warning" => ViolationSeverity::Warning,
            "Minor" => ViolationSeverity::Minor,
            "Major" => ViolationSeverity::Major,
            "Critical" => ViolationSeverity::Critical,
            "Catastrophic" => ViolationSeverity::Catastrophic,
            _ => return Err("Unknown severity level".to_string()),
        };

        let context: HashMap<String, serde_json::Value> = serde_json::from_str(context_json)
            .map_err(|e| format!("Failed to parse context JSON: {}", e))?;

        let violation = self.enforcer.record_violation(
            violation_type,
            severity,
            description.to_string(),
            context,
        );

        serde_json::to_string(&violation)
            .map_err(|e| format!("Failed to serialize violation: {}", e))
    }

    /// Get recent violations
    #[wasm_bindgen]
    pub fn get_recent_violations(&self, limit: usize) -> Result<String, String> {
        let violations = self.enforcer.get_recent_violations(limit);
        
        serde_json::to_string(&violations)
            .map_err(|e| format!("Failed to serialize violations: {}", e))
    }

    /// Perform reality check
    #[wasm_bindgen]
    pub fn perform_reality_check(&self) -> Result<String, String> {
        let result = self.enforcer.perform_reality_check();
        
        serde_json::to_string(&result)
            .map_err(|e| format!("Failed to serialize reality check result: {}", e))
    }

    /// Get violation statistics
    #[wasm_bindgen]
    pub fn get_violation_stats(&self) -> Result<String, String> {
        let stats = self.enforcer.get_violation_stats();
        
        serde_json::to_string(&stats)
            .map_err(|e| format!("Failed to serialize stats: {}", e))
    }

    /// Clear all violations
    #[wasm_bindgen]
    pub fn clear_all_violations(&self) {
        self.enforcer.clear_all_violations();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_violation_recording() {
        let enforcer = DisciplinaryEnforcer::new();
        
        let mut context = HashMap::new();
        context.insert("file".to_string(), serde_json::json!("README.md"));
        
        let violation = enforcer.record_violation(
            ViolationType::DocumentationLoopHallucination,
            ViolationSeverity::Major,
            "Created unnecessary documentation".to_string(),
            context,
        );
        
        assert_eq!(violation.violation_type, ViolationType::DocumentationLoopHallucination);
        assert_eq!(violation.severity, ViolationSeverity::Major);
        assert!(!violation.resolved);
    }

    #[test]
    fn test_violation_stats() {
        let enforcer = DisciplinaryEnforcer::new();
        
        // Record multiple violations
        for _ in 0..3 {
            enforcer.record_violation(
                ViolationType::DocumentationLoopHallucination,
                ViolationSeverity::Minor,
                "Test violation".to_string(),
                HashMap::new(),
            );
        }
        
        let stats = enforcer.get_violation_stats();
        assert_eq!(stats.total_violations, 3);
        assert_eq!(stats.violations_by_type.get("DocumentationLoopHallucination"), Some(&3));
    }

    #[test]
    fn test_reality_check() {
        let enforcer = DisciplinaryEnforcer::new();
        
        // Record violations that should trigger recommendations
        for _ in 0..4 {
            enforcer.record_violation(
                ViolationType::DocumentationLoopHallucination,
                ViolationSeverity::Minor,
                "Documentation loop".to_string(),
                HashMap::new(),
            );
        }
        
        let result = enforcer.perform_reality_check();
        assert!(!result.issues.is_empty());
        assert!(!result.recommendations.is_empty());
        assert_eq!(result.status, RealityCheckStatus::NeedsAttention);
    }
}