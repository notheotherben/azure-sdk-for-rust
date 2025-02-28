#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AadDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AadDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AadDataConnectorProperties {
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "dataTypes", skip_serializing_if = "Option::is_none")]
    pub data_types: Option<AlertsDataTypeOfDataConnector>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AatpDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AatpDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AatpDataConnectorProperties {
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "dataTypes", skip_serializing_if = "Option::is_none")]
    pub data_types: Option<AlertsDataTypeOfDataConnector>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AscDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AscDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AscDataConnectorProperties {
    #[serde(flatten)]
    pub data_connector_with_alerts_properties: DataConnectorWithAlertsProperties,
    #[serde(rename = "subscriptionId", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionRequest {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionRequestProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionPropertiesBase {
    #[serde(rename = "logicAppResourceId")]
    pub logic_app_resource_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionRequestProperties {
    #[serde(flatten)]
    pub action_properties_base: ActionPropertiesBase,
    #[serde(rename = "triggerUri")]
    pub trigger_uri: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionResponse {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionResponseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionResponseProperties {
    #[serde(flatten)]
    pub action_properties_base: ActionPropertiesBase,
    #[serde(rename = "workflowId", skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionsList {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    pub value: Vec<ActionResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRule {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    pub kind: AlertRuleKind,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AlertRuleKind {
    Scheduled,
    MicrosoftSecurityIncidentCreation,
    Fusion,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AlertRuleTemplateStatus {
    Installed,
    Available,
    NotAvailable,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleTemplate {
    #[serde(flatten)]
    pub resource: Resource,
    pub kind: AlertRuleKind,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleTemplateDataSource {
    #[serde(rename = "connectorId", skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[serde(rename = "dataTypes", skip_serializing_if = "Vec::is_empty")]
    pub data_types: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleTemplatesList {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    pub value: Vec<AlertRuleTemplate>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AlertRuleTriggerOperator {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRulesList {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    pub value: Vec<AlertRule>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AlertSeverity {
    High,
    Medium,
    Low,
    Informational,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MicrosoftSecurityProductName {
    #[serde(rename = "Microsoft Cloud App Security")]
    MicrosoftCloudAppSecurity,
    #[serde(rename = "Azure Security Center")]
    AzureSecurityCenter,
    #[serde(rename = "Azure Advanced Threat Protection")]
    AzureAdvancedThreatProtection,
    #[serde(rename = "Azure Active Directory Identity Protection")]
    AzureActiveDirectoryIdentityProtection,
    #[serde(rename = "Azure Security Center for IoT")]
    AzureSecurityCenterForIoT,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsDataTypeOfDataConnector {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<DataConnectorDataTypeCommon>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AttackTactic {
    InitialAccess,
    Execution,
    Persistence,
    PrivilegeEscalation,
    DefenseEvasion,
    CredentialAccess,
    Discovery,
    LateralMovement,
    Collection,
    Exfiltration,
    CommandAndControl,
    Impact,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudTrailDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AwsCloudTrailDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudTrailDataConnectorDataTypes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCloudTrailDataConnectorProperties {
    #[serde(rename = "awsRoleArn", skip_serializing_if = "Option::is_none")]
    pub aws_role_arn: Option<String>,
    #[serde(rename = "dataTypes", skip_serializing_if = "Option::is_none")]
    pub data_types: Option<AwsCloudTrailDataConnectorDataTypes>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bookmark {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BookmarkProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BookmarkList {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    pub value: Vec<Bookmark>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BookmarkProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserInfo>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub query: String,
    #[serde(rename = "queryResult", skip_serializing_if = "Option::is_none")]
    pub query_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(rename = "updatedBy", skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<UserInfo>,
    #[serde(rename = "eventTime", skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    #[serde(rename = "queryStartTime", skip_serializing_if = "Option::is_none")]
    pub query_start_time: Option<String>,
    #[serde(rename = "queryEndTime", skip_serializing_if = "Option::is_none")]
    pub query_end_time: Option<String>,
    #[serde(rename = "incidentInfo", skip_serializing_if = "Option::is_none")]
    pub incident_info: Option<IncidentInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "objectId", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(rename = "userPrincipalName", skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnector {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    pub kind: DataConnectorKind,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectorDataTypeCommon {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<data_connector_data_type_common::State>,
}
pub mod data_connector_data_type_common {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DataConnectorKind {
    AzureActiveDirectory,
    AzureSecurityCenter,
    MicrosoftCloudAppSecurity,
    ThreatIntelligence,
    Office365,
    AmazonWebServicesCloudTrail,
    AzureAdvancedThreatProtection,
    MicrosoftDefenderAdvancedThreatProtection,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectorList {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    pub value: Vec<DataConnector>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectorTenantId {
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectorWithAlertsProperties {
    #[serde(rename = "dataTypes", skip_serializing_if = "Option::is_none")]
    pub data_types: Option<AlertsDataTypeOfDataConnector>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FusionAlertRule {
    #[serde(flatten)]
    pub alert_rule: AlertRule,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<FusionAlertRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FusionAlertRuleProperties {
    #[serde(rename = "alertRuleTemplateName")]
    pub alert_rule_template_name: String,
    #[serde(skip_serializing)]
    pub description: Option<String>,
    #[serde(rename = "displayName", skip_serializing)]
    pub display_name: Option<String>,
    pub enabled: bool,
    #[serde(rename = "lastModifiedUtc", skip_serializing)]
    pub last_modified_utc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<AlertSeverity>,
    #[serde(skip_serializing)]
    pub tactics: Vec<AttackTactic>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FusionAlertRuleTemplate {
    #[serde(flatten)]
    pub alert_rule_template: AlertRuleTemplate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<FusionAlertRuleTemplateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FusionAlertRuleTemplateProperties {
    #[serde(rename = "alertRulesCreatedByTemplateCount", skip_serializing_if = "Option::is_none")]
    pub alert_rules_created_by_template_count: Option<i32>,
    #[serde(rename = "createdDateUTC", skip_serializing)]
    pub created_date_utc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "requiredDataConnectors", skip_serializing_if = "Vec::is_empty")]
    pub required_data_connectors: Vec<AlertRuleTemplateDataSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AlertRuleTemplateStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<AlertSeverity>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tactics: Vec<AttackTactic>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Incident {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<IncidentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentAdditionalData {
    #[serde(rename = "alertsCount", skip_serializing)]
    pub alerts_count: Option<i32>,
    #[serde(rename = "bookmarksCount", skip_serializing)]
    pub bookmarks_count: Option<i32>,
    #[serde(rename = "commentsCount", skip_serializing)]
    pub comments_count: Option<i32>,
    #[serde(rename = "alertProductNames", skip_serializing)]
    pub alert_product_names: Vec<String>,
    #[serde(skip_serializing)]
    pub tactics: Vec<AttackTactic>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentComment {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<IncidentCommentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentCommentList {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    pub value: Vec<IncidentComment>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentCommentProperties {
    #[serde(rename = "createdTimeUtc", skip_serializing)]
    pub created_time_utc: Option<String>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<ClientInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentLabel {
    #[serde(rename = "labelName")]
    pub label_name: String,
    #[serde(rename = "labelType", skip_serializing)]
    pub label_type: Option<incident_label::LabelType>,
}
pub mod incident_label {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LabelType {
        User,
        System,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentList {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    pub value: Vec<Incident>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentOwnerInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "assignedTo", skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    #[serde(rename = "objectId", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(rename = "userPrincipalName", skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentProperties {
    #[serde(rename = "additionalData", skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<IncidentAdditionalData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<incident_properties::Classification>,
    #[serde(rename = "classificationComment", skip_serializing_if = "Option::is_none")]
    pub classification_comment: Option<String>,
    #[serde(rename = "classificationReason", skip_serializing_if = "Option::is_none")]
    pub classification_reason: Option<incident_properties::ClassificationReason>,
    #[serde(rename = "createdTimeUtc", skip_serializing)]
    pub created_time_utc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "firstActivityTimeUtc", skip_serializing_if = "Option::is_none")]
    pub first_activity_time_utc: Option<String>,
    #[serde(rename = "incidentUrl", skip_serializing)]
    pub incident_url: Option<String>,
    #[serde(rename = "incidentNumber", skip_serializing)]
    pub incident_number: Option<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<IncidentLabel>,
    #[serde(rename = "lastActivityTimeUtc", skip_serializing_if = "Option::is_none")]
    pub last_activity_time_utc: Option<String>,
    #[serde(rename = "lastModifiedTimeUtc", skip_serializing)]
    pub last_modified_time_utc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<IncidentOwnerInfo>,
    #[serde(rename = "relatedAnalyticRuleIds", skip_serializing)]
    pub related_analytic_rule_ids: Vec<String>,
    pub severity: incident_properties::Severity,
    pub status: incident_properties::Status,
    pub title: String,
}
pub mod incident_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Classification {
        Undetermined,
        TruePositive,
        BenignPositive,
        FalsePositive,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ClassificationReason {
        SuspiciousActivity,
        SuspiciousButExpected,
        IncorrectAlertLogic,
        InaccurateData,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        High,
        Medium,
        Low,
        Informational,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        New,
        Active,
        Closed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Label {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct McasDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<McasDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct McasDataConnectorDataTypes {
    #[serde(flatten)]
    pub alerts_data_type_of_data_connector: AlertsDataTypeOfDataConnector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<DataConnectorDataTypeCommon>,
    #[serde(rename = "discoveryLogs", skip_serializing_if = "Option::is_none")]
    pub discovery_logs: Option<DataConnectorDataTypeCommon>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct McasDataConnectorProperties {
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "dataTypes", skip_serializing_if = "Option::is_none")]
    pub data_types: Option<McasDataConnectorDataTypes>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MdatpDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<MdatpDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MdatpDataConnectorProperties {
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "dataTypes", skip_serializing_if = "Option::is_none")]
    pub data_types: Option<AlertsDataTypeOfDataConnector>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftSecurityIncidentCreationAlertRule {
    #[serde(flatten)]
    pub alert_rule: AlertRule,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<MicrosoftSecurityIncidentCreationAlertRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftSecurityIncidentCreationAlertRuleCommonProperties {
    #[serde(rename = "displayNamesFilter", skip_serializing_if = "Vec::is_empty")]
    pub display_names_filter: Vec<String>,
    #[serde(rename = "displayNamesExcludeFilter", skip_serializing_if = "Vec::is_empty")]
    pub display_names_exclude_filter: Vec<String>,
    #[serde(rename = "productFilter")]
    pub product_filter: MicrosoftSecurityProductName,
    #[serde(rename = "severitiesFilter", skip_serializing_if = "Vec::is_empty")]
    pub severities_filter: Vec<AlertSeverity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftSecurityIncidentCreationAlertRuleProperties {
    #[serde(flatten)]
    pub microsoft_security_incident_creation_alert_rule_common_properties: MicrosoftSecurityIncidentCreationAlertRuleCommonProperties,
    #[serde(rename = "alertRuleTemplateName", skip_serializing_if = "Option::is_none")]
    pub alert_rule_template_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub enabled: bool,
    #[serde(rename = "lastModifiedUtc", skip_serializing)]
    pub last_modified_utc: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftSecurityIncidentCreationAlertRuleTemplateProperties {
    #[serde(rename = "alertRulesCreatedByTemplateCount", skip_serializing_if = "Option::is_none")]
    pub alert_rules_created_by_template_count: Option<i32>,
    #[serde(rename = "createdDateUTC", skip_serializing)]
    pub created_date_utc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "requiredDataConnectors", skip_serializing_if = "Vec::is_empty")]
    pub required_data_connectors: Vec<AlertRuleTemplateDataSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AlertRuleTemplateStatus>,
    #[serde(rename = "displayNamesFilter", skip_serializing_if = "Vec::is_empty")]
    pub display_names_filter: Vec<String>,
    #[serde(rename = "displayNamesExcludeFilter", skip_serializing_if = "Vec::is_empty")]
    pub display_names_exclude_filter: Vec<String>,
    #[serde(rename = "productFilter")]
    pub product_filter: MicrosoftSecurityProductName,
    #[serde(rename = "severitiesFilter", skip_serializing_if = "Vec::is_empty")]
    pub severities_filter: Vec<AlertSeverity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftSecurityIncidentCreationAlertRuleTemplate {
    #[serde(flatten)]
    pub alert_rule_template: AlertRuleTemplate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<MicrosoftSecurityIncidentCreationAlertRuleTemplateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeConsent {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<OfficeConsentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeConsentList {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    pub value: Vec<OfficeConsent>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeConsentProperties {
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "tenantName", skip_serializing)]
    pub tenant_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<OfficeDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeDataConnectorDataTypes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<serde_json::Value>,
    #[serde(rename = "sharePoint", skip_serializing_if = "Option::is_none")]
    pub share_point: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeDataConnectorProperties {
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "dataTypes", skip_serializing_if = "Option::is_none")]
    pub data_types: Option<OfficeDataConnectorDataTypes>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsList {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceWithEtag {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledAlertRule {
    #[serde(flatten)]
    pub alert_rule: AlertRule,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduledAlertRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledAlertRuleCommonProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "queryFrequency", skip_serializing_if = "Option::is_none")]
    pub query_frequency: Option<String>,
    #[serde(rename = "queryPeriod", skip_serializing_if = "Option::is_none")]
    pub query_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<AlertSeverity>,
    #[serde(rename = "triggerOperator", skip_serializing_if = "Option::is_none")]
    pub trigger_operator: Option<AlertRuleTriggerOperator>,
    #[serde(rename = "triggerThreshold", skip_serializing_if = "Option::is_none")]
    pub trigger_threshold: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledAlertRuleProperties {
    #[serde(flatten)]
    pub scheduled_alert_rule_common_properties: ScheduledAlertRuleCommonProperties,
    #[serde(rename = "alertRuleTemplateName", skip_serializing_if = "Option::is_none")]
    pub alert_rule_template_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub enabled: bool,
    #[serde(rename = "lastModifiedUtc", skip_serializing)]
    pub last_modified_utc: Option<String>,
    #[serde(rename = "suppressionDuration")]
    pub suppression_duration: String,
    #[serde(rename = "suppressionEnabled")]
    pub suppression_enabled: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tactics: Vec<AttackTactic>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledAlertRuleTemplateProperties {
    #[serde(rename = "alertRulesCreatedByTemplateCount", skip_serializing_if = "Option::is_none")]
    pub alert_rules_created_by_template_count: Option<i32>,
    #[serde(rename = "createdDateUTC", skip_serializing)]
    pub created_date_utc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "requiredDataConnectors", skip_serializing_if = "Vec::is_empty")]
    pub required_data_connectors: Vec<AlertRuleTemplateDataSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AlertRuleTemplateStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "queryFrequency", skip_serializing_if = "Option::is_none")]
    pub query_frequency: Option<String>,
    #[serde(rename = "queryPeriod", skip_serializing_if = "Option::is_none")]
    pub query_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<AlertSeverity>,
    #[serde(rename = "triggerOperator", skip_serializing_if = "Option::is_none")]
    pub trigger_operator: Option<AlertRuleTriggerOperator>,
    #[serde(rename = "triggerThreshold", skip_serializing_if = "Option::is_none")]
    pub trigger_threshold: Option<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tactics: Vec<AttackTactic>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledAlertRuleTemplate {
    #[serde(flatten)]
    pub alert_rule_template: AlertRuleTemplate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduledAlertRuleTemplateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    #[serde(flatten)]
    pub resource_with_etag: ResourceWithEtag,
    pub kind: SettingsKind,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SettingsKind {
    UebaSettings,
    ToggleSettings,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiDataConnector {
    #[serde(flatten)]
    pub data_connector: DataConnector,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<TiDataConnectorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiDataConnectorDataTypes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indicators: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TiDataConnectorProperties {
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "tipLookbackPeriod", skip_serializing_if = "Option::is_none")]
    pub tip_lookback_period: Option<String>,
    #[serde(rename = "dataTypes", skip_serializing_if = "Option::is_none")]
    pub data_types: Option<TiDataConnectorDataTypes>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreatIntelligence {
    #[serde(skip_serializing)]
    pub confidence: Option<f64>,
    #[serde(rename = "providerName", skip_serializing)]
    pub provider_name: Option<String>,
    #[serde(rename = "reportLink", skip_serializing)]
    pub report_link: Option<String>,
    #[serde(rename = "threatDescription", skip_serializing)]
    pub threat_description: Option<String>,
    #[serde(rename = "threatName", skip_serializing)]
    pub threat_name: Option<String>,
    #[serde(rename = "threatType", skip_serializing)]
    pub threat_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToggleSettings {
    #[serde(flatten)]
    pub settings: Settings,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ToggleSettingsProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToggleSettingsProperties {
    #[serde(rename = "isEnabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UebaSettings {
    #[serde(flatten)]
    pub settings: Settings,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UebaSettingsProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UebaSettingsProperties {
    #[serde(rename = "atpLicenseStatus", skip_serializing)]
    pub atp_license_status: Option<ueba_settings_properties::AtpLicenseStatus>,
    #[serde(rename = "isEnabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "statusInMcas", skip_serializing)]
    pub status_in_mcas: Option<ueba_settings_properties::StatusInMcas>,
}
pub mod ueba_settings_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AtpLicenseStatus {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StatusInMcas {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    #[serde(skip_serializing)]
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "objectId")]
    pub object_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentInfo {
    #[serde(rename = "incidentId", skip_serializing_if = "Option::is_none")]
    pub incident_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<incident_info::Severity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "relationName", skip_serializing_if = "Option::is_none")]
    pub relation_name: Option<String>,
}
pub mod incident_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        Critical,
        High,
        Medium,
        Low,
        Informational,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ErrorResponse>,
    #[serde(rename = "additionalInfo", skip_serializing)]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub info: Option<serde_json::Value>,
}
