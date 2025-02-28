#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub location: Option<String>,
    #[serde(skip_serializing)]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeAnalyticsAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataLakeAnalyticsAccountProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeAnalyticsAccountBasic {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataLakeAnalyticsAccountPropertiesBasic>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeAnalyticsAccountProperties {
    #[serde(flatten)]
    pub data_lake_analytics_account_properties_basic: DataLakeAnalyticsAccountPropertiesBasic,
    #[serde(rename = "defaultDataLakeStoreAccount", skip_serializing)]
    pub default_data_lake_store_account: Option<String>,
    #[serde(rename = "dataLakeStoreAccounts", skip_serializing)]
    pub data_lake_store_accounts: Vec<DataLakeStoreAccountInformation>,
    #[serde(rename = "publicDataLakeStoreAccounts", skip_serializing_if = "Vec::is_empty")]
    pub public_data_lake_store_accounts: Vec<DataLakeStoreAccountInformation>,
    #[serde(rename = "storageAccounts", skip_serializing)]
    pub storage_accounts: Vec<StorageAccountInformation>,
    #[serde(rename = "computePolicies", skip_serializing)]
    pub compute_policies: Vec<ComputePolicy>,
    #[serde(rename = "hiveMetastores", skip_serializing)]
    pub hive_metastores: Vec<HiveMetastore>,
    #[serde(rename = "virtualNetworkRules", skip_serializing)]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
    #[serde(rename = "firewallRules", skip_serializing)]
    pub firewall_rules: Vec<FirewallRule>,
    #[serde(rename = "firewallState", skip_serializing_if = "Option::is_none")]
    pub firewall_state: Option<data_lake_analytics_account_properties::FirewallState>,
    #[serde(rename = "firewallAllowAzureIps", skip_serializing_if = "Option::is_none")]
    pub firewall_allow_azure_ips: Option<data_lake_analytics_account_properties::FirewallAllowAzureIps>,
    #[serde(rename = "newTier", skip_serializing_if = "Option::is_none")]
    pub new_tier: Option<data_lake_analytics_account_properties::NewTier>,
    #[serde(rename = "currentTier", skip_serializing)]
    pub current_tier: Option<data_lake_analytics_account_properties::CurrentTier>,
    #[serde(rename = "maxJobCount", skip_serializing_if = "Option::is_none")]
    pub max_job_count: Option<i32>,
    #[serde(rename = "systemMaxJobCount", skip_serializing)]
    pub system_max_job_count: Option<i32>,
    #[serde(rename = "maxDegreeOfParallelism", skip_serializing_if = "Option::is_none")]
    pub max_degree_of_parallelism: Option<i32>,
    #[serde(rename = "systemMaxDegreeOfParallelism", skip_serializing)]
    pub system_max_degree_of_parallelism: Option<i32>,
    #[serde(rename = "maxDegreeOfParallelismPerJob", skip_serializing_if = "Option::is_none")]
    pub max_degree_of_parallelism_per_job: Option<i32>,
    #[serde(rename = "minPriorityPerJob", skip_serializing)]
    pub min_priority_per_job: Option<i32>,
    #[serde(rename = "queryStoreRetention", skip_serializing_if = "Option::is_none")]
    pub query_store_retention: Option<i32>,
    #[serde(rename = "debugDataAccessLevel", skip_serializing)]
    pub debug_data_access_level: Option<data_lake_analytics_account_properties::DebugDataAccessLevel>,
}
pub mod data_lake_analytics_account_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallState {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallAllowAzureIps {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NewTier {
        Consumption,
        #[serde(rename = "Commitment_100AUHours")]
        Commitment100auHours,
        #[serde(rename = "Commitment_500AUHours")]
        Commitment500auHours,
        #[serde(rename = "Commitment_1000AUHours")]
        Commitment1000auHours,
        #[serde(rename = "Commitment_5000AUHours")]
        Commitment5000auHours,
        #[serde(rename = "Commitment_10000AUHours")]
        Commitment10000auHours,
        #[serde(rename = "Commitment_50000AUHours")]
        Commitment50000auHours,
        #[serde(rename = "Commitment_100000AUHours")]
        Commitment100000auHours,
        #[serde(rename = "Commitment_500000AUHours")]
        Commitment500000auHours,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CurrentTier {
        Consumption,
        #[serde(rename = "Commitment_100AUHours")]
        Commitment100auHours,
        #[serde(rename = "Commitment_500AUHours")]
        Commitment500auHours,
        #[serde(rename = "Commitment_1000AUHours")]
        Commitment1000auHours,
        #[serde(rename = "Commitment_5000AUHours")]
        Commitment5000auHours,
        #[serde(rename = "Commitment_10000AUHours")]
        Commitment10000auHours,
        #[serde(rename = "Commitment_50000AUHours")]
        Commitment50000auHours,
        #[serde(rename = "Commitment_100000AUHours")]
        Commitment100000auHours,
        #[serde(rename = "Commitment_500000AUHours")]
        Commitment500000auHours,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DebugDataAccessLevel {
        All,
        Customer,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeAnalyticsAccountPropertiesBasic {
    #[serde(rename = "accountId", skip_serializing)]
    pub account_id: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<data_lake_analytics_account_properties_basic::ProvisioningState>,
    #[serde(skip_serializing)]
    pub state: Option<data_lake_analytics_account_properties_basic::State>,
    #[serde(rename = "creationTime", skip_serializing)]
    pub creation_time: Option<String>,
    #[serde(rename = "lastModifiedTime", skip_serializing)]
    pub last_modified_time: Option<String>,
    #[serde(skip_serializing)]
    pub endpoint: Option<String>,
}
pub mod data_lake_analytics_account_properties_basic {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Failed,
        Creating,
        Running,
        Succeeded,
        Patching,
        Suspending,
        Resuming,
        Deleting,
        Deleted,
        Undeleting,
        Canceled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Active,
        Suspended,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeAnalyticsAccountListResult {
    #[serde(skip_serializing)]
    pub value: Vec<DataLakeAnalyticsAccountBasic>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeStoreAccountInformation {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataLakeStoreAccountInformationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeStoreAccountInformationProperties {
    #[serde(skip_serializing)]
    pub suffix: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeStoreAccountInformationListResult {
    #[serde(skip_serializing)]
    pub value: Vec<DataLakeStoreAccountInformation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountInformation {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountInformationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountInformationProperties {
    #[serde(skip_serializing)]
    pub suffix: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountInformationListResult {
    #[serde(skip_serializing)]
    pub value: Vec<StorageAccountInformation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageContainer {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageContainerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageContainerProperties {
    #[serde(rename = "lastModifiedTime", skip_serializing)]
    pub last_modified_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageContainerListResult {
    #[serde(skip_serializing)]
    pub value: Vec<StorageContainer>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SasTokenInformation {
    #[serde(rename = "accessToken", skip_serializing)]
    pub access_token: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SasTokenInformationListResult {
    #[serde(skip_serializing)]
    pub value: Vec<SasTokenInformation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputePolicy {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ComputePolicyProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputePolicyProperties {
    #[serde(rename = "objectId", skip_serializing)]
    pub object_id: Option<String>,
    #[serde(rename = "objectType", skip_serializing)]
    pub object_type: Option<compute_policy_properties::ObjectType>,
    #[serde(rename = "maxDegreeOfParallelismPerJob", skip_serializing)]
    pub max_degree_of_parallelism_per_job: Option<i32>,
    #[serde(rename = "minPriorityPerJob", skip_serializing)]
    pub min_priority_per_job: Option<i32>,
}
pub mod compute_policy_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ObjectType {
        User,
        Group,
        ServicePrincipal,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputePolicyListResult {
    #[serde(skip_serializing)]
    pub value: Vec<ComputePolicy>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRule {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<FirewallRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRuleProperties {
    #[serde(rename = "startIpAddress", skip_serializing)]
    pub start_ip_address: Option<String>,
    #[serde(rename = "endIpAddress", skip_serializing)]
    pub end_ip_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRuleListResult {
    #[serde(skip_serializing)]
    pub value: Vec<FirewallRule>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VirtualNetworkRuleState {
    Active,
    NetworkSourceDeleted,
    Failed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRuleProperties {
    #[serde(rename = "subnetId", skip_serializing)]
    pub subnet_id: Option<String>,
    #[serde(rename = "virtualNetworkRuleState", skip_serializing)]
    pub virtual_network_rule_state: Option<VirtualNetworkRuleState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRuleListResult {
    #[serde(skip_serializing)]
    pub value: Vec<VirtualNetworkRule>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HiveMetastore {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HiveMetastoreProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HiveMetastoreProperties {
    #[serde(rename = "serverUri", skip_serializing)]
    pub server_uri: Option<String>,
    #[serde(rename = "databaseName", skip_serializing)]
    pub database_name: Option<String>,
    #[serde(rename = "runtimeVersion", skip_serializing)]
    pub runtime_version: Option<String>,
    #[serde(rename = "userName", skip_serializing)]
    pub user_name: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    #[serde(rename = "nestedResourceProvisioningState", skip_serializing)]
    pub nested_resource_provisioning_state: Option<NestedResourceProvisioningState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HiveMetastoreListResult {
    #[serde(skip_serializing)]
    pub value: Vec<HiveMetastore>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NestedResourceProvisioningState {
    Succeeded,
    Canceled,
    Failed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationMetaPropertyInfo>,
    #[serde(skip_serializing)]
    pub origin: Option<operation::Origin>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplay {
    #[serde(skip_serializing)]
    pub provider: Option<String>,
    #[serde(skip_serializing)]
    pub resource: Option<String>,
    #[serde(skip_serializing)]
    pub operation: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationMetaMetricAvailabilitiesSpecification {
    #[serde(rename = "timeGrain", skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[serde(rename = "blobDuration", skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationMetaMetricSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayDescription", skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "aggregationType", skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub availabilities: Vec<OperationMetaMetricAvailabilitiesSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationMetaLogSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "blobDuration", skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationMetaServiceSpecification {
    #[serde(rename = "metricSpecifications", skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<OperationMetaMetricSpecification>,
    #[serde(rename = "logSpecifications", skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<OperationMetaLogSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationMetaPropertyInfo {
    #[serde(rename = "serviceSpecification", skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<OperationMetaServiceSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapabilityInformation {
    #[serde(rename = "subscriptionId", skip_serializing)]
    pub subscription_id: Option<String>,
    #[serde(skip_serializing)]
    pub state: Option<capability_information::State>,
    #[serde(rename = "maxAccountCount", skip_serializing)]
    pub max_account_count: Option<i32>,
    #[serde(rename = "accountCount", skip_serializing)]
    pub account_count: Option<i32>,
    #[serde(rename = "migrationState", skip_serializing)]
    pub migration_state: Option<bool>,
}
pub mod capability_information {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Registered,
        Suspended,
        Deleted,
        Unregistered,
        Warned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAvailabilityInformation {
    #[serde(rename = "nameAvailable", skip_serializing)]
    pub name_available: Option<bool>,
    #[serde(skip_serializing)]
    pub reason: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateDataLakeAnalyticsAccountParameters {
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub properties: CreateDataLakeAnalyticsAccountProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateDataLakeAnalyticsAccountProperties {
    #[serde(rename = "defaultDataLakeStoreAccount")]
    pub default_data_lake_store_account: String,
    #[serde(rename = "dataLakeStoreAccounts")]
    pub data_lake_store_accounts: Vec<AddDataLakeStoreWithAccountParameters>,
    #[serde(rename = "storageAccounts", skip_serializing_if = "Vec::is_empty")]
    pub storage_accounts: Vec<AddStorageAccountWithAccountParameters>,
    #[serde(rename = "computePolicies", skip_serializing_if = "Vec::is_empty")]
    pub compute_policies: Vec<CreateComputePolicyWithAccountParameters>,
    #[serde(rename = "firewallRules", skip_serializing_if = "Vec::is_empty")]
    pub firewall_rules: Vec<CreateFirewallRuleWithAccountParameters>,
    #[serde(rename = "firewallState", skip_serializing_if = "Option::is_none")]
    pub firewall_state: Option<create_data_lake_analytics_account_properties::FirewallState>,
    #[serde(rename = "firewallAllowAzureIps", skip_serializing_if = "Option::is_none")]
    pub firewall_allow_azure_ips: Option<create_data_lake_analytics_account_properties::FirewallAllowAzureIps>,
    #[serde(rename = "newTier", skip_serializing_if = "Option::is_none")]
    pub new_tier: Option<create_data_lake_analytics_account_properties::NewTier>,
    #[serde(rename = "maxJobCount", skip_serializing_if = "Option::is_none")]
    pub max_job_count: Option<i32>,
    #[serde(rename = "maxDegreeOfParallelism", skip_serializing_if = "Option::is_none")]
    pub max_degree_of_parallelism: Option<i32>,
    #[serde(rename = "maxDegreeOfParallelismPerJob", skip_serializing_if = "Option::is_none")]
    pub max_degree_of_parallelism_per_job: Option<i32>,
    #[serde(rename = "minPriorityPerJob", skip_serializing_if = "Option::is_none")]
    pub min_priority_per_job: Option<i32>,
    #[serde(rename = "queryStoreRetention", skip_serializing_if = "Option::is_none")]
    pub query_store_retention: Option<i32>,
}
pub mod create_data_lake_analytics_account_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallState {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallAllowAzureIps {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NewTier {
        Consumption,
        #[serde(rename = "Commitment_100AUHours")]
        Commitment100auHours,
        #[serde(rename = "Commitment_500AUHours")]
        Commitment500auHours,
        #[serde(rename = "Commitment_1000AUHours")]
        Commitment1000auHours,
        #[serde(rename = "Commitment_5000AUHours")]
        Commitment5000auHours,
        #[serde(rename = "Commitment_10000AUHours")]
        Commitment10000auHours,
        #[serde(rename = "Commitment_50000AUHours")]
        Commitment50000auHours,
        #[serde(rename = "Commitment_100000AUHours")]
        Commitment100000auHours,
        #[serde(rename = "Commitment_500000AUHours")]
        Commitment500000auHours,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateDataLakeAnalyticsAccountParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateDataLakeAnalyticsAccountProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateDataLakeAnalyticsAccountProperties {
    #[serde(rename = "dataLakeStoreAccounts", skip_serializing_if = "Vec::is_empty")]
    pub data_lake_store_accounts: Vec<UpdateDataLakeStoreWithAccountParameters>,
    #[serde(rename = "storageAccounts", skip_serializing_if = "Vec::is_empty")]
    pub storage_accounts: Vec<UpdateStorageAccountWithAccountParameters>,
    #[serde(rename = "computePolicies", skip_serializing_if = "Vec::is_empty")]
    pub compute_policies: Vec<UpdateComputePolicyWithAccountParameters>,
    #[serde(rename = "firewallRules", skip_serializing_if = "Vec::is_empty")]
    pub firewall_rules: Vec<UpdateFirewallRuleWithAccountParameters>,
    #[serde(rename = "firewallState", skip_serializing_if = "Option::is_none")]
    pub firewall_state: Option<update_data_lake_analytics_account_properties::FirewallState>,
    #[serde(rename = "firewallAllowAzureIps", skip_serializing_if = "Option::is_none")]
    pub firewall_allow_azure_ips: Option<update_data_lake_analytics_account_properties::FirewallAllowAzureIps>,
    #[serde(rename = "newTier", skip_serializing_if = "Option::is_none")]
    pub new_tier: Option<update_data_lake_analytics_account_properties::NewTier>,
    #[serde(rename = "maxJobCount", skip_serializing_if = "Option::is_none")]
    pub max_job_count: Option<i32>,
    #[serde(rename = "maxDegreeOfParallelism", skip_serializing_if = "Option::is_none")]
    pub max_degree_of_parallelism: Option<i32>,
    #[serde(rename = "maxDegreeOfParallelismPerJob", skip_serializing_if = "Option::is_none")]
    pub max_degree_of_parallelism_per_job: Option<i32>,
    #[serde(rename = "minPriorityPerJob", skip_serializing_if = "Option::is_none")]
    pub min_priority_per_job: Option<i32>,
    #[serde(rename = "queryStoreRetention", skip_serializing_if = "Option::is_none")]
    pub query_store_retention: Option<i32>,
}
pub mod update_data_lake_analytics_account_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallState {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallAllowAzureIps {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NewTier {
        Consumption,
        #[serde(rename = "Commitment_100AUHours")]
        Commitment100auHours,
        #[serde(rename = "Commitment_500AUHours")]
        Commitment500auHours,
        #[serde(rename = "Commitment_1000AUHours")]
        Commitment1000auHours,
        #[serde(rename = "Commitment_5000AUHours")]
        Commitment5000auHours,
        #[serde(rename = "Commitment_10000AUHours")]
        Commitment10000auHours,
        #[serde(rename = "Commitment_50000AUHours")]
        Commitment50000auHours,
        #[serde(rename = "Commitment_100000AUHours")]
        Commitment100000auHours,
        #[serde(rename = "Commitment_500000AUHours")]
        Commitment500000auHours,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddDataLakeStoreParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AddDataLakeStoreProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddDataLakeStoreWithAccountParameters {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AddDataLakeStoreProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddDataLakeStoreProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateDataLakeStoreWithAccountParameters {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateDataLakeStoreProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateDataLakeStoreProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddStorageAccountParameters {
    pub properties: AddStorageAccountProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddStorageAccountWithAccountParameters {
    pub name: String,
    pub properties: AddStorageAccountProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddStorageAccountProperties {
    #[serde(rename = "accessKey")]
    pub access_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateStorageAccountParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateStorageAccountProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateStorageAccountWithAccountParameters {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateStorageAccountProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateStorageAccountProperties {
    #[serde(rename = "accessKey", skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateComputePolicyParameters {
    pub properties: CreateOrUpdateComputePolicyProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateComputePolicyWithAccountParameters {
    pub name: String,
    pub properties: CreateOrUpdateComputePolicyProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateComputePolicyProperties {
    #[serde(rename = "objectId")]
    pub object_id: String,
    #[serde(rename = "objectType")]
    pub object_type: create_or_update_compute_policy_properties::ObjectType,
    #[serde(rename = "maxDegreeOfParallelismPerJob", skip_serializing_if = "Option::is_none")]
    pub max_degree_of_parallelism_per_job: Option<i32>,
    #[serde(rename = "minPriorityPerJob", skip_serializing_if = "Option::is_none")]
    pub min_priority_per_job: Option<i32>,
}
pub mod create_or_update_compute_policy_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ObjectType {
        User,
        Group,
        ServicePrincipal,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateComputePolicyParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateComputePolicyProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateComputePolicyWithAccountParameters {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateComputePolicyProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateComputePolicyProperties {
    #[serde(rename = "objectId", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(rename = "objectType", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<update_compute_policy_properties::ObjectType>,
    #[serde(rename = "maxDegreeOfParallelismPerJob", skip_serializing_if = "Option::is_none")]
    pub max_degree_of_parallelism_per_job: Option<i32>,
    #[serde(rename = "minPriorityPerJob", skip_serializing_if = "Option::is_none")]
    pub min_priority_per_job: Option<i32>,
}
pub mod update_compute_policy_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ObjectType {
        User,
        Group,
        ServicePrincipal,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateFirewallRuleParameters {
    pub properties: CreateOrUpdateFirewallRuleProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFirewallRuleWithAccountParameters {
    pub name: String,
    pub properties: CreateOrUpdateFirewallRuleProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateFirewallRuleProperties {
    #[serde(rename = "startIpAddress")]
    pub start_ip_address: String,
    #[serde(rename = "endIpAddress")]
    pub end_ip_address: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateFirewallRuleParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateFirewallRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateFirewallRuleWithAccountParameters {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateFirewallRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateFirewallRuleProperties {
    #[serde(rename = "startIpAddress", skip_serializing_if = "Option::is_none")]
    pub start_ip_address: Option<String>,
    #[serde(rename = "endIpAddress", skip_serializing_if = "Option::is_none")]
    pub end_ip_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameters {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: check_name_availability_parameters::Type,
}
pub mod check_name_availability_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.DataLakeAnalytics/accounts")]
        MicrosoftDataLakeAnalyticsAccounts,
    }
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
