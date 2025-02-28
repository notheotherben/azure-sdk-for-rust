#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExport {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataExportProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExportListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataExport>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExportProperties {
    #[serde(rename = "dataExportId", skip_serializing_if = "Option::is_none")]
    pub data_export_id: Option<String>,
    #[serde(rename = "tableNames")]
    pub table_names: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Destination {
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<destination::Type>,
    #[serde(rename = "metaData", skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<DestinationMetaData>,
}
pub mod destination {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        StorageAccount,
        EventHub,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DestinationMetaData {
    #[serde(rename = "eventHubName", skip_serializing_if = "Option::is_none")]
    pub event_hub_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DataSourceKind {
    WindowsEvent,
    WindowsPerformanceCounter,
    #[serde(rename = "IISLogs")]
    IisLogs,
    LinuxSyslog,
    LinuxSyslogCollection,
    LinuxPerformanceObject,
    LinuxPerformanceCollection,
    CustomLog,
    CustomLogCollection,
    AzureAuditLog,
    AzureActivityLog,
    GenericDataSource,
    ChangeTrackingCustomPath,
    ChangeTrackingPath,
    ChangeTrackingServices,
    ChangeTrackingDataTypeConfiguration,
    ChangeTrackingDefaultRegistry,
    ChangeTrackingRegistry,
    ChangeTrackingLinuxPath,
    LinuxChangeTrackingPath,
    ChangeTrackingContentLocation,
    WindowsTelemetry,
    Office365,
    SecurityWindowsBaselineConfiguration,
    SecurityCenterSecurityWindowsBaselineConfiguration,
    SecurityEventCollectionConfiguration,
    SecurityInsightsSecurityEventCollectionConfiguration,
    ImportComputerGroup,
    NetworkMonitoring,
    Itsm,
    DnsAnalytics,
    ApplicationInsights,
    SqlDataClassification,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: Object,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    pub kind: DataSourceKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSourceFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<DataSourceKind>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSourceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataSource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Object {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntelligencePack {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedServiceProperties {
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "writeAccessResourceId", skip_serializing_if = "Option::is_none")]
    pub write_access_resource_id: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<linked_service_properties::ProvisioningState>,
}
pub mod linked_service_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Deleting,
        ProvisioningAccount,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedService {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: LinkedServiceProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedServiceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LinkedService>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedStorageAccountsProperties {
    #[serde(rename = "dataSourceType", skip_serializing)]
    pub data_source_type: Option<linked_storage_accounts_properties::DataSourceType>,
    #[serde(rename = "storageAccountIds", skip_serializing_if = "Vec::is_empty")]
    pub storage_account_ids: Vec<String>,
}
pub mod linked_storage_accounts_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataSourceType {
        CustomLogs,
        AzureWatson,
        Query,
        Alerts,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedStorageAccountsResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: LinkedStorageAccountsProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedStorageAccountsListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LinkedStorageAccountsResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementGroupProperties {
    #[serde(rename = "serverCount", skip_serializing_if = "Option::is_none")]
    pub server_count: Option<i32>,
    #[serde(rename = "isGateway", skip_serializing_if = "Option::is_none")]
    pub is_gateway: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "dataReceived", skip_serializing_if = "Option::is_none")]
    pub data_received: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementGroupProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceListManagementGroupsResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagementGroup>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedKeys {
    #[serde(rename = "primarySharedKey", skip_serializing_if = "Option::is_none")]
    pub primary_shared_key: Option<String>,
    #[serde(rename = "secondarySharedKey", skip_serializing_if = "Option::is_none")]
    pub secondary_shared_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageMetric {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "currentValue", skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(rename = "nextResetTime", skip_serializing_if = "Option::is_none")]
    pub next_reset_time: Option<String>,
    #[serde(rename = "quotaPeriod", skip_serializing_if = "Option::is_none")]
    pub quota_period: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceListUsagesResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UsageMetric>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceSku {
    pub name: workspace_sku::Name,
    #[serde(rename = "capacityReservationLevel", skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_level: Option<i32>,
    #[serde(rename = "maxCapacityReservationLevel", skip_serializing)]
    pub max_capacity_reservation_level: Option<i32>,
    #[serde(rename = "lastSkuUpdate", skip_serializing)]
    pub last_sku_update: Option<String>,
}
pub mod workspace_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Free,
        Standard,
        Premium,
        PerNode,
        #[serde(rename = "PerGB2018")]
        PerGb2018,
        Standalone,
        CapacityReservation,
        #[serde(rename = "LACluster")]
        LaCluster,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceCapping {
    #[serde(rename = "dailyQuotaGb", skip_serializing_if = "Option::is_none")]
    pub daily_quota_gb: Option<f64>,
    #[serde(rename = "quotaNextResetTime", skip_serializing)]
    pub quota_next_reset_time: Option<String>,
    #[serde(rename = "dataIngestionStatus", skip_serializing)]
    pub data_ingestion_status: Option<workspace_capping::DataIngestionStatus>,
}
pub mod workspace_capping {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataIngestionStatus {
        RespectQuota,
        ForceOn,
        ForceOff,
        OverQuota,
        SubscriptionSuspended,
        ApproachingQuota,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceProperties {
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workspace_properties::ProvisioningState>,
    #[serde(rename = "customerId", skip_serializing)]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<WorkspaceSku>,
    #[serde(rename = "retentionInDays", skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i32>,
    #[serde(rename = "workspaceCapping", skip_serializing_if = "Option::is_none")]
    pub workspace_capping: Option<WorkspaceCapping>,
    #[serde(rename = "createdDate", skip_serializing)]
    pub created_date: Option<String>,
    #[serde(rename = "modifiedDate", skip_serializing)]
    pub modified_date: Option<String>,
    #[serde(rename = "publicNetworkAccessForIngestion", skip_serializing_if = "Option::is_none")]
    pub public_network_access_for_ingestion: Option<PublicNetworkAccessType>,
    #[serde(rename = "publicNetworkAccessForQuery", skip_serializing_if = "Option::is_none")]
    pub public_network_access_for_query: Option<PublicNetworkAccessType>,
    #[serde(rename = "forceCmkForQuery", skip_serializing_if = "Option::is_none")]
    pub force_cmk_for_query: Option<bool>,
    #[serde(rename = "privateLinkScopedResources", skip_serializing)]
    pub private_link_scoped_resources: Vec<PrivateLinkScopedResource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<WorkspaceFeatures>,
}
pub mod workspace_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Succeeded,
        Failed,
        Canceled,
        Deleting,
        ProvisioningAccount,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceFeatures {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkScopedResource {
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "scopeId", skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
    #[serde(rename = "eTag", skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspacePatch {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workspace>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PublicNetworkAccessType {
    Enabled,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterProperties {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(rename = "clusterId", skip_serializing)]
    pub cluster_id: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<cluster_properties::ProvisioningState>,
    #[serde(rename = "keyVaultProperties", skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
}
pub mod cluster_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Succeeded,
        Failed,
        Canceled,
        Deleting,
        ProvisioningAccount,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterPatchProperties {
    #[serde(rename = "keyVaultProperties", skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterPatchProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<ClusterSku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<ClusterSku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterListResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cluster>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultProperties {
    #[serde(rename = "keyVaultUri", skip_serializing_if = "Option::is_none")]
    pub key_vault_uri: Option<String>,
    #[serde(rename = "keyName", skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(rename = "keyVersion", skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterSku {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cluster_sku::Name>,
}
pub mod cluster_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        CapacityReservation,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: identity::Type,
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccount {
    pub id: String,
    pub key: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageInsightStatus {
    pub state: storage_insight_status::State,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
pub mod storage_insight_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "OK")]
        Ok,
        #[serde(rename = "ERROR")]
        Error,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageInsightProperties {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub containers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<String>,
    #[serde(rename = "storageAccount")]
    pub storage_account: StorageAccount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<StorageInsightStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageInsight {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageInsightProperties>,
    #[serde(rename = "eTag", skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageInsightListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageInsight>,
    #[serde(rename = "@odata.nextLink", skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub value: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedSearchProperties {
    pub category: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub query: String,
    #[serde(rename = "functionAlias", skip_serializing_if = "Option::is_none")]
    pub function_alias: Option<String>,
    #[serde(rename = "functionParameters", skip_serializing_if = "Option::is_none")]
    pub function_parameters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedSearch {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    pub properties: SavedSearchProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedSearchesListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SavedSearch>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableServiceTier {
    #[serde(rename = "serviceTier", skip_serializing)]
    pub service_tier: Option<available_service_tier::ServiceTier>,
    #[serde(skip_serializing)]
    pub enabled: Option<bool>,
    #[serde(rename = "minimumRetention", skip_serializing)]
    pub minimum_retention: Option<i64>,
    #[serde(rename = "maximumRetention", skip_serializing)]
    pub maximum_retention: Option<i64>,
    #[serde(rename = "defaultRetention", skip_serializing)]
    pub default_retention: Option<i64>,
    #[serde(rename = "capacityReservationLevel", skip_serializing)]
    pub capacity_reservation_level: Option<i64>,
    #[serde(rename = "lastSkuUpdate", skip_serializing)]
    pub last_sku_update: Option<String>,
}
pub mod available_service_tier {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServiceTier {
        Free,
        Standard,
        Premium,
        PerNode,
        #[serde(rename = "PerGB2018")]
        PerGb2018,
        Standalone,
        CapacityReservation,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoreSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "numberOfDocuments")]
    pub number_of_documents: i64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSort {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<search_sort::Order>,
}
pub mod search_sort {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Order {
        #[serde(rename = "asc")]
        Asc,
        #[serde(rename = "desc")]
        Desc,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchMetadataSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchMetadata {
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "resultType", skip_serializing_if = "Option::is_none")]
    pub result_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "coreSummaries", skip_serializing_if = "Vec::is_empty")]
    pub core_summaries: Vec<CoreSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "eTag", skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sort: Vec<SearchSort>,
    #[serde(rename = "requestTime", skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,
    #[serde(rename = "aggregatedValueField", skip_serializing_if = "Option::is_none")]
    pub aggregated_value_field: Option<String>,
    #[serde(rename = "aggregatedGroupingFields", skip_serializing_if = "Option::is_none")]
    pub aggregated_grouping_fields: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<SearchMetadataSchema>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSchemaValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub indexed: bool,
    pub stored: bool,
    pub facet: bool,
    #[serde(rename = "ownerType", skip_serializing_if = "Vec::is_empty")]
    pub owner_type: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchGetSchemaResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SearchMetadata>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SearchSchemaValue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspacePurgeBody {
    pub table: String,
    pub filters: Vec<WorkspacePurgeBodyFilters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspacePurgeBodyFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspacePurgeResponse {
    #[serde(rename = "operationId")]
    pub operation_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspacePurgeStatusResponse {
    pub status: workspace_purge_status_response::Status,
}
pub mod workspace_purge_status_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "completed")]
        Completed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableProperties {
    #[serde(rename = "retentionInDays", skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Table {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<TableProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TablesListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Table>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ErrorDetail>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
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
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureEntityResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing)]
    pub etag: Option<String>,
}
