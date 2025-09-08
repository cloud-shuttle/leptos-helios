//! TDD Tests for Comprehensive Data Governance System
//!
//! This module tests the enhanced data governance capabilities including:
//! - Data classification and sensitivity management
//! - Data lineage tracking and transformation history
//! - Privacy rules and compliance (GDPR, CCPA, etc.)
//! - Risk assessment and mitigation
//! - Compliance framework management
//! - Export compliance and data protection

use leptos_helios::security::*;
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::test]
async fn test_data_classification_and_policies() {
    let governance = DataGovernance::new();

    // Classify data sources
    governance
        .classify_data("customer_pii", DataClassification::Confidential)
        .await
        .unwrap();

    governance
        .classify_data("public_analytics", DataClassification::Public)
        .await
        .unwrap();

    // Add data policies
    let policy = DataPolicy {
        id: "policy_1".to_string(),
        name: "Confidential Data Policy".to_string(),
        classification: DataClassification::Confidential,
        retention_days: Some(2555), // 7 years
        geographic_restrictions: vec!["EU".to_string(), "US".to_string()],
        allowed_exports: vec!["encrypted_pdf".to_string(), "encrypted_excel".to_string()],
        required_approvals: vec!["data_officer".to_string()],
        encryption_required: true,
        anonymization_required: false,
        access_controls: vec!["rbac".to_string(), "mfa".to_string()],
        audit_required: true,
    };

    governance.add_data_policy(policy).await.unwrap();

    // Test export compliance
    let user = User {
        id: "user1".to_string(),
        username: "data_officer".to_string(),
        email: "officer@company.com".to_string(),
        display_name: "Data Officer".to_string(),
        roles: {
            let mut roles = HashSet::new();
            roles.insert("data_officer".to_string());
            roles
        },
        permissions: HashSet::new(),
        groups: HashSet::new(),
        attributes: HashMap::new(),
        created_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        last_login: None,
        is_active: true,
    };

    // Should pass for encrypted export with proper role
    let result = governance
        .check_export_compliance("customer_pii", "encrypted_pdf", &user)
        .await;
    assert!(result.is_ok());

    // Should fail for non-encrypted export
    let result = governance
        .check_export_compliance("customer_pii", "pdf", &user)
        .await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        SecurityError::DataGovernanceViolation(_)
    ));

    // Should fail for user without required role
    let unauthorized_user = User {
        id: "user2".to_string(),
        username: "regular_user".to_string(),
        email: "user@company.com".to_string(),
        display_name: "Regular User".to_string(),
        roles: HashSet::new(),
        permissions: HashSet::new(),
        groups: HashSet::new(),
        attributes: HashMap::new(),
        created_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        last_login: None,
        is_active: true,
    };

    let result = governance
        .check_export_compliance("customer_pii", "encrypted_pdf", &unauthorized_user)
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_data_lineage_tracking() {
    let governance = DataGovernance::new();

    // Track data lineage
    let transformation = DataTransformation {
        id: "trans_1".to_string(),
        operation: "anonymize_pii".to_string(),
        parameters: {
            let mut params = HashMap::new();
            params.insert(
                "fields".to_string(),
                serde_json::Value::Array(vec![
                    serde_json::Value::String("email".to_string()),
                    serde_json::Value::String("phone".to_string()),
                ]),
            );
            params.insert(
                "method".to_string(),
                serde_json::Value::String("hash".to_string()),
            );
            params
        },
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        user_id: "data_engineer".to_string(),
        impact_level: TransformationImpact::High,
    };

    governance
        .track_data_lineage(
            "customer_data_v2".to_string(),
            "raw_customer_data".to_string(),
            transformation,
        )
        .await
        .unwrap();

    // Retrieve lineage
    let lineage = governance
        .get_data_lineage("customer_data_v2")
        .await
        .unwrap();

    assert!(lineage.is_some());
    let lineage = lineage.unwrap();
    assert_eq!(lineage.data_id, "customer_data_v2");
    assert_eq!(lineage.source, "raw_customer_data");
    assert_eq!(lineage.transformations.len(), 1);
    assert_eq!(lineage.transformations[0].operation, "anonymize_pii");
    assert!(matches!(
        lineage.transformations[0].impact_level,
        TransformationImpact::High
    ));
}

#[tokio::test]
async fn test_privacy_rules_and_compliance() {
    let governance = DataGovernance::new();

    // Add GDPR privacy rule
    let gdpr_rule = PrivacyRule {
        id: "gdpr_1".to_string(),
        name: "GDPR Data Minimization".to_string(),
        rule_type: PrivacyRuleType::DataMinimization,
        conditions: vec![PrivacyCondition {
            field: "data_classification".to_string(),
            operator: ConditionOperator::Equals,
            value: serde_json::Value::String("Confidential".to_string()),
        }],
        actions: vec![
            PrivacyAction::Anonymize,
            PrivacyAction::LogAccess,
            PrivacyAction::RequireConsent,
        ],
        jurisdiction: "EU".to_string(),
        framework: "GDPR".to_string(),
    };

    governance.add_privacy_rule(gdpr_rule).await.unwrap();

    // Add CCPA privacy rule
    let ccpa_rule = PrivacyRule {
        id: "ccpa_1".to_string(),
        name: "CCPA Right to Delete".to_string(),
        rule_type: PrivacyRuleType::RightToErasure,
        conditions: vec![PrivacyCondition {
            field: "data_classification".to_string(),
            operator: ConditionOperator::Equals,
            value: serde_json::Value::String("Confidential".to_string()),
        }],
        actions: vec![PrivacyAction::AutoDelete, PrivacyAction::NotifyDataOwner],
        jurisdiction: "California".to_string(),
        framework: "CCPA".to_string(),
    };

    governance.add_privacy_rule(ccpa_rule).await.unwrap();

    // Evaluate privacy rules
    let user = User {
        id: "user1".to_string(),
        username: "data_processor".to_string(),
        email: "processor@company.com".to_string(),
        display_name: "Data Processor".to_string(),
        roles: HashSet::new(),
        permissions: HashSet::new(),
        groups: HashSet::new(),
        attributes: HashMap::new(),
        created_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        last_login: None,
        is_active: true,
    };

    let actions = governance
        .evaluate_privacy_rules(&DataClassification::Confidential, &user, "process")
        .await
        .unwrap();

    assert!(!actions.is_empty());
    assert!(actions.contains(&PrivacyAction::Anonymize));
    assert!(actions.contains(&PrivacyAction::LogAccess));
    assert!(actions.contains(&PrivacyAction::RequireConsent));
}

#[tokio::test]
async fn test_compliance_framework_management() {
    let governance = DataGovernance::new();

    // Add GDPR compliance framework
    let gdpr_framework = ComplianceFramework {
        id: "gdpr_framework".to_string(),
        name: "General Data Protection Regulation".to_string(),
        version: "2018".to_string(),
        jurisdiction: "EU".to_string(),
        requirements: vec![
            ComplianceRequirement {
                id: "art_5_1_c".to_string(),
                title: "Data Minimization".to_string(),
                description:
                    "Personal data shall be adequate, relevant and limited to what is necessary"
                        .to_string(),
                category: "Data Protection Principles".to_string(),
                mandatory: true,
                evidence_required: vec![
                    "data_inventory".to_string(),
                    "purpose_limitation_doc".to_string(),
                ],
            },
            ComplianceRequirement {
                id: "art_32".to_string(),
                title: "Security of Processing".to_string(),
                description: "Appropriate technical and organizational measures to ensure security"
                    .to_string(),
                category: "Security".to_string(),
                mandatory: true,
                evidence_required: vec![
                    "security_assessment".to_string(),
                    "encryption_policy".to_string(),
                ],
            },
        ],
        controls: vec![ComplianceControl {
            id: "ctrl_1".to_string(),
            requirement_id: "art_5_1_c".to_string(),
            name: "Data Classification System".to_string(),
            control_type: ControlType::Preventive,
            implementation: "Automated data classification based on content analysis".to_string(),
            frequency: AssessmentFrequency::Continuous,
            owner: "Data Protection Officer".to_string(),
        }],
        assessment_criteria: vec![AssessmentCriteria {
            id: "criteria_1".to_string(),
            control_id: "ctrl_1".to_string(),
            criteria: "All personal data is properly classified within 24 hours".to_string(),
            measurement_method: "Automated monitoring and reporting".to_string(),
            target_value: serde_json::Value::Number(100.into()),
            acceptable_deviation: 0.0,
        }],
    };

    governance
        .add_compliance_framework(gdpr_framework)
        .await
        .unwrap();

    // Conduct compliance assessment
    let assessment = governance
        .assess_compliance("gdpr_framework", "customer_data")
        .await
        .unwrap();

    assert_eq!(assessment.framework_id, "gdpr_framework");
    assert_eq!(assessment.data_id, "customer_data");
    assert!(matches!(
        assessment.overall_compliance,
        ComplianceLevel::Compliant
    ));
    assert_eq!(assessment.requirement_assessments.len(), 2);

    // Check that mandatory requirements are marked as compliant
    for req_assessment in &assessment.requirement_assessments {
        assert!(matches!(req_assessment.status, ComplianceStatus::Compliant));
    }
}

#[tokio::test]
async fn test_risk_assessment_and_mitigation() {
    let governance = DataGovernance::new();

    // Classify data first
    governance
        .classify_data("financial_records", DataClassification::Restricted)
        .await
        .unwrap();

    // Conduct risk assessment
    let assessment = governance
        .conduct_risk_assessment("financial_records".to_string(), "risk_assessor".to_string())
        .await
        .unwrap();

    assert_eq!(assessment.data_id, "financial_records");
    assert_eq!(assessment.assessor, "risk_assessor");
    assert!(matches!(assessment.risk_level, RiskLevel::High)); // Restricted data = High risk
    assert!(!assessment.identified_risks.is_empty());
    assert!(!assessment.mitigation_measures.is_empty());
    assert!(matches!(assessment.residual_risk, RiskLevel::Low)); // After mitigation

    // Check identified risks
    let risk = &assessment.identified_risks[0];
    assert_eq!(risk.id, "risk_1");
    assert!(matches!(risk.risk_type, RiskType::DataBreach));
    assert!(matches!(risk.likelihood, RiskLevel::Medium));
    assert!(matches!(risk.impact, RiskLevel::High));

    // Check mitigation measures
    let mitigation = &assessment.mitigation_measures[0];
    assert_eq!(mitigation.id, "mit_1");
    assert_eq!(mitigation.risk_id, "risk_1");
    assert!(matches!(
        mitigation.implementation_status,
        ImplementationStatus::Implemented
    ));
    assert_eq!(mitigation.effectiveness, 0.8);
    assert_eq!(mitigation.owner, "Security Team");

    // Retrieve stored assessment
    let stored_assessment = governance
        .get_risk_assessment("financial_records")
        .await
        .unwrap();

    assert!(stored_assessment.is_some());
    let stored = stored_assessment.unwrap();
    assert_eq!(stored.data_id, "financial_records");
    assert_eq!(stored.assessor, "risk_assessor");
}

#[tokio::test]
async fn test_data_governance_policy_management() {
    let governance = DataGovernance::new();

    // Add multiple policies for different classifications
    let policies = vec![
        DataPolicy {
            id: "public_policy".to_string(),
            name: "Public Data Policy".to_string(),
            classification: DataClassification::Public,
            retention_days: Some(365),
            geographic_restrictions: vec![],
            allowed_exports: vec!["pdf".to_string(), "csv".to_string(), "json".to_string()],
            required_approvals: vec![],
            encryption_required: false,
            anonymization_required: false,
            access_controls: vec![],
            audit_required: false,
        },
        DataPolicy {
            id: "internal_policy".to_string(),
            name: "Internal Data Policy".to_string(),
            classification: DataClassification::Internal,
            retention_days: Some(1095), // 3 years
            geographic_restrictions: vec!["US".to_string()],
            allowed_exports: vec!["pdf".to_string(), "excel".to_string()],
            required_approvals: vec!["manager".to_string()],
            encryption_required: false,
            anonymization_required: false,
            access_controls: vec!["rbac".to_string()],
            audit_required: true,
        },
        DataPolicy {
            id: "topsecret_policy".to_string(),
            name: "Top Secret Data Policy".to_string(),
            classification: DataClassification::TopSecret,
            retention_days: Some(2555), // 7 years
            geographic_restrictions: vec!["US".to_string()],
            allowed_exports: vec!["encrypted_pdf".to_string()],
            required_approvals: vec!["security_officer".to_string(), "data_officer".to_string()],
            encryption_required: true,
            anonymization_required: true,
            access_controls: vec![
                "rbac".to_string(),
                "mfa".to_string(),
                "ip_whitelist".to_string(),
            ],
            audit_required: true,
        },
    ];

    for policy in policies {
        governance.add_data_policy(policy).await.unwrap();
    }

    // Retrieve all policies
    let all_policies = governance.get_data_policies().await.unwrap();
    assert_eq!(all_policies.len(), 3);

    // Verify policy details
    let topsecret_policy = all_policies
        .iter()
        .find(|p| p.classification == DataClassification::TopSecret)
        .unwrap();

    assert_eq!(topsecret_policy.name, "Top Secret Data Policy");
    assert!(topsecret_policy.encryption_required);
    assert!(topsecret_policy.anonymization_required);
    assert!(topsecret_policy.audit_required);
    assert_eq!(topsecret_policy.required_approvals.len(), 2);
    assert!(topsecret_policy
        .required_approvals
        .contains(&"security_officer".to_string()));
    assert!(topsecret_policy
        .required_approvals
        .contains(&"data_officer".to_string()));
}

#[tokio::test]
async fn test_comprehensive_data_governance_workflow() {
    let governance = DataGovernance::new();

    // Step 1: Classify sensitive data
    governance
        .classify_data("patient_health_records", DataClassification::Restricted)
        .await
        .unwrap();

    // Step 2: Add appropriate policy
    let health_policy = DataPolicy {
        id: "hipaa_policy".to_string(),
        name: "HIPAA Health Records Policy".to_string(),
        classification: DataClassification::Restricted,
        retention_days: Some(2555), // 7 years
        geographic_restrictions: vec!["US".to_string()],
        allowed_exports: vec!["encrypted_pdf".to_string()],
        required_approvals: vec!["hipaa_officer".to_string()],
        encryption_required: true,
        anonymization_required: true,
        access_controls: vec!["rbac".to_string(), "mfa".to_string()],
        audit_required: true,
    };

    governance.add_data_policy(health_policy).await.unwrap();

    // Step 3: Add HIPAA compliance framework
    let hipaa_framework = ComplianceFramework {
        id: "hipaa_framework".to_string(),
        name: "Health Insurance Portability and Accountability Act".to_string(),
        version: "1996".to_string(),
        jurisdiction: "US".to_string(),
        requirements: vec![ComplianceRequirement {
            id: "164_312_a_1".to_string(),
            title: "Access Control".to_string(),
            description:
                "Implement technical policies and procedures for electronic information systems"
                    .to_string(),
            category: "Administrative Safeguards".to_string(),
            mandatory: true,
            evidence_required: vec!["access_control_policy".to_string()],
        }],
        controls: vec![],
        assessment_criteria: vec![],
    };

    governance
        .add_compliance_framework(hipaa_framework)
        .await
        .unwrap();

    // Step 4: Track data lineage
    let transformation = DataTransformation {
        id: "deid_1".to_string(),
        operation: "de_identify_phi".to_string(),
        parameters: {
            let mut params = HashMap::new();
            params.insert(
                "method".to_string(),
                serde_json::Value::String("safe_harbor".to_string()),
            );
            params.insert(
                "fields_removed".to_string(),
                serde_json::Value::Array(vec![
                    serde_json::Value::String("name".to_string()),
                    serde_json::Value::String("ssn".to_string()),
                    serde_json::Value::String("address".to_string()),
                ]),
            );
            params
        },
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        user_id: "data_analyst".to_string(),
        impact_level: TransformationImpact::Critical,
    };

    governance
        .track_data_lineage(
            "deidentified_health_data".to_string(),
            "patient_health_records".to_string(),
            transformation,
        )
        .await
        .unwrap();

    // Step 5: Conduct risk assessment
    let risk_assessment = governance
        .conduct_risk_assessment(
            "patient_health_records".to_string(),
            "compliance_officer".to_string(),
        )
        .await
        .unwrap();

    assert!(matches!(risk_assessment.risk_level, RiskLevel::High));
    assert!(!risk_assessment.identified_risks.is_empty());

    // Step 6: Conduct compliance assessment
    let compliance_assessment = governance
        .assess_compliance("hipaa_framework", "patient_health_records")
        .await
        .unwrap();

    assert_eq!(compliance_assessment.framework_id, "hipaa_framework");
    assert_eq!(compliance_assessment.data_id, "patient_health_records");

    // Step 7: Test export compliance
    let hipaa_officer = User {
        id: "hipaa_officer".to_string(),
        username: "hipaa_officer".to_string(),
        email: "hipaa@healthcare.com".to_string(),
        display_name: "HIPAA Officer".to_string(),
        roles: {
            let mut roles = HashSet::new();
            roles.insert("hipaa_officer".to_string());
            roles
        },
        permissions: HashSet::new(),
        groups: HashSet::new(),
        attributes: HashMap::new(),
        created_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        last_login: None,
        is_active: true,
    };

    // Should pass for encrypted export with proper authorization
    let result = governance
        .check_export_compliance("patient_health_records", "encrypted_pdf", &hipaa_officer)
        .await;
    assert!(result.is_ok());

    // Should fail for non-encrypted export
    let result = governance
        .check_export_compliance("patient_health_records", "pdf", &hipaa_officer)
        .await;
    assert!(result.is_err());

    // Step 8: Verify data lineage
    let lineage = governance
        .get_data_lineage("deidentified_health_data")
        .await
        .unwrap();

    assert!(lineage.is_some());
    let lineage = lineage.unwrap();
    assert_eq!(lineage.source, "patient_health_records");
    assert_eq!(lineage.transformations.len(), 1);
    assert_eq!(lineage.transformations[0].operation, "de_identify_phi");
    assert!(matches!(
        lineage.transformations[0].impact_level,
        TransformationImpact::Critical
    ));
}
