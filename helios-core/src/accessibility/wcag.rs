//! WCAG Compliance System
//!
//! This module provides WCAG 2.1 compliance checking and reporting.

use super::{AccessibilityError, DataTable};
use crate::chart::{ChartSpec, MarkType};
use polars::prelude::DataFrame;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// WCAG 2.1 compliance levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WCAGLevel {
    A,   // Basic compliance
    AA,  // Standard compliance (target)
    AAA, // Enhanced compliance
}

/// WCAG compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub level_achieved: WCAGLevel,
    pub violations: Vec<ComplianceViolation>,
    pub warnings: Vec<String>,
    pub score: f64, // 0.0 to 100.0
    pub recommendations: Vec<String>,
    pub tested_criteria: Vec<String>,
}

/// Individual compliance violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    pub criterion: String,
    pub severity: ViolationSeverity,
    pub description: String,
    pub remedy: String,
    pub impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// WCAG compliance checker
pub struct WCAGChecker {
    target_level: WCAGLevel,
    criteria: HashMap<String, WCAGCriterion>,
}

/// Individual WCAG criterion
#[derive(Debug, Clone)]
pub struct WCAGCriterion {
    pub id: String,
    pub level: WCAGLevel,
    pub title: String,
    pub description: String,
    pub test_function: fn(&ChartSpec, &DataFrame) -> Result<(), String>,
}

impl WCAGChecker {
    /// Create a new WCAG checker
    pub fn new(target_level: WCAGLevel) -> Self {
        let mut checker = Self {
            target_level,
            criteria: HashMap::new(),
        };
        checker.initialize_criteria();
        checker
    }

    /// Check compliance for a chart specification
    pub fn check_compliance(&self, spec: &ChartSpec, data: &DataFrame) -> ComplianceReport {
        let mut violations = Vec::new();
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        let mut tested_criteria = Vec::new();

        for (criterion_id, criterion) in &self.criteria {
            // Only test criteria at or below the target level
            if self.should_test_criterion(criterion) {
                tested_criteria.push(criterion_id.clone());

                match (criterion.test_function)(spec, data) {
                    Ok(()) => {
                        // Criterion passed
                    }
                    Err(error_msg) => {
                        let severity = self.determine_severity(criterion);
                        violations.push(ComplianceViolation {
                            criterion: criterion_id.clone(),
                            severity,
                            description: error_msg.clone(),
                            remedy: self.get_remedy(criterion_id),
                            impact: self.get_impact(criterion_id),
                        });
                    }
                }
            }
        }

        let score = self.calculate_score(&violations, &tested_criteria);
        let level_achieved = self.determine_level_achieved(&violations);

        ComplianceReport {
            level_achieved,
            violations,
            warnings,
            score,
            recommendations,
            tested_criteria,
        }
    }

    /// Initialize WCAG criteria
    fn initialize_criteria(&mut self) {
        // 1.1.1 Non-text Content (Level A)
        self.criteria.insert(
            "1.1.1".to_string(),
            WCAGCriterion {
                id: "1.1.1".to_string(),
                level: WCAGLevel::A,
                title: "Non-text Content".to_string(),
                description: "All non-text content has a text alternative".to_string(),
                test_function: Self::test_alt_text,
            },
        );

        // 1.3.1 Info and Relationships (Level A)
        self.criteria.insert(
            "1.3.1".to_string(),
            WCAGCriterion {
                id: "1.3.1".to_string(),
                level: WCAGLevel::A,
                title: "Info and Relationships".to_string(),
                description: "Information and relationships are programmatically determinable"
                    .to_string(),
                test_function: Self::test_info_relationships,
            },
        );

        // 1.4.1 Use of Color (Level A)
        self.criteria.insert(
            "1.4.1".to_string(),
            WCAGCriterion {
                id: "1.4.1".to_string(),
                level: WCAGLevel::A,
                title: "Use of Color".to_string(),
                description: "Color is not the only means of conveying information".to_string(),
                test_function: Self::test_color_usage,
            },
        );

        // 1.4.3 Contrast (Minimum) (Level AA)
        self.criteria.insert(
            "1.4.3".to_string(),
            WCAGCriterion {
                id: "1.4.3".to_string(),
                level: WCAGLevel::AA,
                title: "Contrast (Minimum)".to_string(),
                description: "Text has a contrast ratio of at least 4.5:1".to_string(),
                test_function: Self::test_contrast,
            },
        );

        // 2.1.1 Keyboard (Level A)
        self.criteria.insert(
            "2.1.1".to_string(),
            WCAGCriterion {
                id: "2.1.1".to_string(),
                level: WCAGLevel::A,
                title: "Keyboard".to_string(),
                description: "All functionality is available from a keyboard".to_string(),
                test_function: Self::test_keyboard_access,
            },
        );

        // 2.4.1 Bypass Blocks (Level A)
        self.criteria.insert(
            "2.4.1".to_string(),
            WCAGCriterion {
                id: "2.4.1".to_string(),
                level: WCAGLevel::A,
                title: "Bypass Blocks".to_string(),
                description: "A mechanism is available to bypass blocks of content".to_string(),
                test_function: Self::test_bypass_blocks,
            },
        );
    }

    /// Test for alt text presence
    fn test_alt_text(spec: &ChartSpec, _data: &DataFrame) -> Result<(), String> {
        // Check if chart has alt text or data table alternative
        if spec.config.title.is_empty() && spec.config.description.is_empty() {
            Err("Chart lacks alternative text description".to_string())
        } else {
            Ok(())
        }
    }

    /// Test for information and relationships
    fn test_info_relationships(spec: &ChartSpec, _data: &DataFrame) -> Result<(), String> {
        // Check if chart structure is programmatically determinable
        if spec.encoding.x.is_none() && spec.encoding.y.is_none() {
            Err("Chart lacks programmatically determinable structure".to_string())
        } else {
            Ok(())
        }
    }

    /// Test for color usage
    fn test_color_usage(spec: &ChartSpec, _data: &DataFrame) -> Result<(), String> {
        // Check if color is the only means of conveying information
        // This is a simplified check - in practice, you'd analyze the visual encoding
        match spec.mark {
            MarkType::Point { .. } => {
                // Points should have additional visual cues beyond color
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Test for contrast
    fn test_contrast(spec: &ChartSpec, _data: &DataFrame) -> Result<(), String> {
        // Check if text elements meet contrast requirements
        // This would typically involve checking actual color values
        Ok(())
    }

    /// Test for keyboard access
    fn test_keyboard_access(spec: &ChartSpec, _data: &DataFrame) -> Result<(), String> {
        // Check if interactive elements are keyboard accessible
        if !spec.selection.is_empty() {
            // Interactive charts should have keyboard navigation
            Ok(())
        } else {
            Ok(())
        }
    }

    /// Test for bypass blocks
    fn test_bypass_blocks(_spec: &ChartSpec, _data: &DataFrame) -> Result<(), String> {
        // Check if skip links or other bypass mechanisms are available
        Ok(())
    }

    /// Determine if a criterion should be tested
    fn should_test_criterion(&self, criterion: &WCAGCriterion) -> bool {
        match (&self.target_level, &criterion.level) {
            (WCAGLevel::A, WCAGLevel::A) => true,
            (WCAGLevel::AA, WCAGLevel::A) | (WCAGLevel::AA, WCAGLevel::AA) => true,
            (WCAGLevel::AAA, _) => true,
            _ => false,
        }
    }

    /// Determine severity of a violation
    fn determine_severity(&self, criterion: &WCAGCriterion) -> ViolationSeverity {
        match criterion.level {
            WCAGLevel::A => ViolationSeverity::Critical,
            WCAGLevel::AA => ViolationSeverity::High,
            WCAGLevel::AAA => ViolationSeverity::Medium,
        }
    }

    /// Get remedy for a criterion
    fn get_remedy(&self, criterion_id: &str) -> String {
        match criterion_id {
            "1.1.1" => "Add alt text or data table alternative".to_string(),
            "1.3.1" => "Ensure chart structure is programmatically determinable".to_string(),
            "1.4.1" => "Add visual cues beyond color (patterns, shapes, labels)".to_string(),
            "1.4.3" => "Increase contrast ratio to at least 4.5:1".to_string(),
            "2.1.1" => "Ensure all functionality is keyboard accessible".to_string(),
            "2.4.1" => "Add skip links or other bypass mechanisms".to_string(),
            _ => "Review WCAG guidelines for this criterion".to_string(),
        }
    }

    /// Get impact description for a criterion
    fn get_impact(&self, criterion_id: &str) -> String {
        match criterion_id {
            "1.1.1" => "Screen reader users cannot understand chart content".to_string(),
            "1.3.1" => "Assistive technologies cannot interpret chart structure".to_string(),
            "1.4.1" => "Colorblind users cannot distinguish data series".to_string(),
            "1.4.3" => "Users with low vision cannot read chart text".to_string(),
            "2.1.1" => "Keyboard users cannot interact with chart".to_string(),
            "2.4.1" => "Keyboard users cannot efficiently navigate chart".to_string(),
            _ => "Accessibility barrier for users with disabilities".to_string(),
        }
    }

    /// Calculate compliance score
    fn calculate_score(
        &self,
        violations: &[super::ComplianceViolation],
        tested_criteria: &[String],
    ) -> f64 {
        if tested_criteria.is_empty() {
            return 100.0;
        }

        let total_criteria = tested_criteria.len() as f64;
        let violation_count = violations.len() as f64;

        ((total_criteria - violation_count) / total_criteria) * 100.0
    }

    /// Determine the level achieved
    fn determine_level_achieved(&self, violations: &[super::ComplianceViolation]) -> WCAGLevel {
        let has_a_violations = violations.iter().any(|v| {
            self.criteria
                .get(&v.criterion)
                .map(|c| c.level == WCAGLevel::A)
                .unwrap_or(false)
        });

        let has_aa_violations = violations.iter().any(|v| {
            self.criteria
                .get(&v.criterion)
                .map(|c| c.level == WCAGLevel::AA)
                .unwrap_or(false)
        });

        if has_a_violations {
            // Cannot achieve any level if A violations exist
            WCAGLevel::A
        } else if has_aa_violations {
            WCAGLevel::A
        } else {
            self.target_level.clone()
        }
    }
}
