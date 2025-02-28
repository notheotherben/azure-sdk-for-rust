#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CognitiveServicesAccountKind {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: SkuName,
    #[serde(skip_serializing)]
    pub tier: Option<sku::Tier>,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Free,
        Standard,
        Premium,
        Enterprise,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuName {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CognitiveServicesAccount {
    #[serde(skip_serializing)]
    pub etag: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<CognitiveServicesAccountKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CognitiveServicesAccountProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CognitiveServicesAccountListResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing)]
    pub value: Vec<CognitiveServicesAccount>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CognitiveServicesAccountProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<cognitive_services_account_properties::ProvisioningState>,
    #[serde(skip_serializing)]
    pub endpoint: Option<String>,
    #[serde(rename = "internalId", skip_serializing)]
    pub internal_id: Option<String>,
    #[serde(skip_serializing)]
    pub capabilities: Vec<SkuCapability>,
    #[serde(rename = "isMigrated", skip_serializing)]
    pub is_migrated: Option<bool>,
    #[serde(rename = "skuChangeInfo", skip_serializing_if = "Option::is_none")]
    pub sku_change_info: Option<CognitiveServicesAccountSkuChangeInfo>,
    #[serde(rename = "customSubDomainName", skip_serializing_if = "Option::is_none")]
    pub custom_sub_domain_name: Option<String>,
    #[serde(rename = "networkAcls", skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "userOwnedStorage", skip_serializing_if = "Vec::is_empty")]
    pub user_owned_storage: Vec<UserOwnedStorage>,
    #[serde(rename = "privateEndpointConnections", skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[serde(rename = "publicNetworkAccess", skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<cognitive_services_account_properties::PublicNetworkAccess>,
    #[serde(rename = "apiProperties", skip_serializing_if = "Option::is_none")]
    pub api_properties: Option<CognitiveServicesAccountApiProperties>,
    #[serde(rename = "dateCreated", skip_serializing)]
    pub date_created: Option<String>,
}
pub mod cognitive_services_account_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        #[serde(rename = "ResolvingDNS")]
        ResolvingDns,
        Moving,
        Deleting,
        Succeeded,
        Failed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CognitiveServicesAccountApiProperties {
    #[serde(rename = "qnaRuntimeEndpoint", skip_serializing_if = "Option::is_none")]
    pub qna_runtime_endpoint: Option<String>,
    #[serde(rename = "qnaAzureSearchEndpointKey", skip_serializing_if = "Option::is_none")]
    pub qna_azure_search_endpoint_key: Option<String>,
    #[serde(rename = "qnaAzureSearchEndpointId", skip_serializing_if = "Option::is_none")]
    pub qna_azure_search_endpoint_id: Option<String>,
    #[serde(rename = "statisticsEnabled", skip_serializing_if = "Option::is_none")]
    pub statistics_enabled: Option<bool>,
    #[serde(rename = "eventHubConnectionString", skip_serializing_if = "Option::is_none")]
    pub event_hub_connection_string: Option<String>,
    #[serde(rename = "storageAccountConnectionString", skip_serializing_if = "Option::is_none")]
    pub storage_account_connection_string: Option<String>,
    #[serde(rename = "aadClientId", skip_serializing_if = "Option::is_none")]
    pub aad_client_id: Option<String>,
    #[serde(rename = "aadTenantId", skip_serializing_if = "Option::is_none")]
    pub aad_tenant_id: Option<String>,
    #[serde(rename = "superUser", skip_serializing_if = "Option::is_none")]
    pub super_user: Option<String>,
    #[serde(rename = "websiteName", skip_serializing_if = "Option::is_none")]
    pub website_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CognitiveServicesAccountKeys {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key2: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CognitiveServicesAccountSkuChangeInfo {
    #[serde(rename = "countOfDowngrades", skip_serializing)]
    pub count_of_downgrades: Option<f64>,
    #[serde(rename = "countOfUpgradesAfterDowngrades", skip_serializing)]
    pub count_of_upgrades_after_downgrades: Option<f64>,
    #[serde(rename = "lastChangeDate", skip_serializing)]
    pub last_change_date: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateKeyParameters {
    #[serde(rename = "keyName")]
    pub key_name: regenerate_key_parameters::KeyName,
}
pub mod regenerate_key_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyName {
        Key1,
        Key2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CognitiveServicesAccountEnumerateSkusResult {
    #[serde(skip_serializing)]
    pub value: Vec<CognitiveServicesResourceAndSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CognitiveServicesResourceAndSku {
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsagesResult {
    #[serde(skip_serializing)]
    pub value: Vec<Usage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    #[serde(skip_serializing)]
    pub unit: Option<UnitType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
    #[serde(rename = "quotaPeriod", skip_serializing)]
    pub quota_period: Option<String>,
    #[serde(skip_serializing)]
    pub limit: Option<f64>,
    #[serde(rename = "currentValue", skip_serializing)]
    pub current_value: Option<f64>,
    #[serde(rename = "nextResetTime", skip_serializing)]
    pub next_reset_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<usage::Status>,
}
pub mod usage {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Included,
        Blocked,
        InOverage,
        Unknown,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricName {
    #[serde(skip_serializing)]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", skip_serializing)]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UnitType {
    Count,
    Bytes,
    Seconds,
    Percent,
    CountPerSecond,
    BytesPerSecond,
    Milliseconds,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationEntityListResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationEntity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplayInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckSkuAvailabilityParameter {
    pub skus: Vec<SkuName>,
    pub kind: CognitiveServicesAccountKind,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckSkuAvailabilityResultList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CheckSkuAvailabilityResult>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckSkuAvailabilityResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<CognitiveServicesAccountKind>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "skuName", skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<SkuName>,
    #[serde(rename = "skuAvailable", skip_serializing_if = "Option::is_none")]
    pub sku_available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckDomainAvailabilityParameter {
    #[serde(rename = "subdomainName")]
    pub subdomain_name: String,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckDomainAvailabilityResult {
    #[serde(rename = "isSubdomainAvailable", skip_serializing_if = "Option::is_none")]
    pub is_subdomain_available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "subdomainName", skip_serializing_if = "Option::is_none")]
    pub subdomain_name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkuRestrictions {
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<resource_sku_restrictions::Type>,
    #[serde(skip_serializing)]
    pub values: Vec<String>,
    #[serde(rename = "restrictionInfo", skip_serializing_if = "Option::is_none")]
    pub restriction_info: Option<ResourceSkuRestrictionInfo>,
    #[serde(rename = "reasonCode", skip_serializing)]
    pub reason_code: Option<resource_sku_restrictions::ReasonCode>,
}
pub mod resource_sku_restrictions {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Location,
        Zone,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReasonCode {
        QuotaId,
        NotAvailableForSubscription,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSku {
    #[serde(rename = "resourceType", skip_serializing)]
    pub resource_type: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub tier: Option<String>,
    #[serde(skip_serializing)]
    pub kind: Option<String>,
    #[serde(skip_serializing)]
    pub locations: Vec<String>,
    #[serde(skip_serializing)]
    pub restrictions: Vec<ResourceSkuRestrictions>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkuRestrictionInfo {
    #[serde(skip_serializing)]
    pub locations: Vec<String>,
    #[serde(skip_serializing)]
    pub zones: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkusResult {
    pub value: Vec<ResourceSku>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRuleSet {
    #[serde(rename = "defaultAction", skip_serializing_if = "Option::is_none")]
    pub default_action: Option<network_rule_set::DefaultAction>,
    #[serde(rename = "ipRules", skip_serializing_if = "Vec::is_empty")]
    pub ip_rules: Vec<IpRule>,
    #[serde(rename = "virtualNetworkRules", skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
}
pub mod network_rule_set {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DefaultAction {
        Allow,
        Deny,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpRule {
    pub value: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "ignoreMissingVnetServiceEndpoint", skip_serializing_if = "Option::is_none")]
    pub ignore_missing_vnet_service_endpoint: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "userAssignedIdentities", skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        None,
        SystemAssigned,
        UserAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAssignedIdentity {
    #[serde(rename = "principalId", skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Encryption {
    #[serde(rename = "keyVaultProperties", skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
    #[serde(rename = "keySource", skip_serializing_if = "Option::is_none")]
    pub key_source: Option<encryption::KeySource>,
}
pub mod encryption {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeySource {
        #[serde(rename = "Microsoft.CognitiveServices")]
        MicrosoftCognitiveServices,
        #[serde(rename = "Microsoft.KeyVault")]
        MicrosoftKeyVault,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultProperties {
    #[serde(rename = "keyName", skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(rename = "keyVersion", skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
    #[serde(rename = "keyVaultUri", skip_serializing_if = "Option::is_none")]
    pub key_vault_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserOwnedStorage {
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
    #[serde(skip_serializing)]
    pub etag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[serde(rename = "groupIds", skip_serializing_if = "Vec::is_empty")]
    pub group_ids: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpoint {
    #[serde(skip_serializing)]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "actionsRequired", skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    Disconnected,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceProperties {
    #[serde(rename = "groupId", skip_serializing)]
    pub group_id: Option<String>,
    #[serde(rename = "displayName", skip_serializing)]
    pub display_name: Option<String>,
    #[serde(rename = "requiredMembers", skip_serializing)]
    pub required_members: Vec<String>,
    #[serde(rename = "requiredZoneNames", skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
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
