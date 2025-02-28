#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<services_properties::ProvisioningState>,
    #[serde(rename = "accessPolicies", skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<ServiceAccessPoliciesInfo>,
    #[serde(rename = "cosmosDbConfiguration", skip_serializing_if = "Option::is_none")]
    pub cosmos_db_configuration: Option<ServiceCosmosDbConfigurationInfo>,
    #[serde(rename = "authenticationConfiguration", skip_serializing_if = "Option::is_none")]
    pub authentication_configuration: Option<ServiceAuthenticationConfigurationInfo>,
    #[serde(rename = "corsConfiguration", skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<ServiceCorsConfigurationInfo>,
    #[serde(rename = "exportConfiguration", skip_serializing_if = "Option::is_none")]
    pub export_configuration: Option<ServiceExportConfigurationInfo>,
    #[serde(rename = "privateEndpointConnections", skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[serde(rename = "publicNetworkAccess", skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<services_properties::PublicNetworkAccess>,
}
pub mod services_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Deleting,
        Succeeded,
        Creating,
        Accepted,
        Verifying,
        Updating,
        Failed,
        Canceled,
        Deprovisioned,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
    }
}
pub type ServiceAccessPoliciesInfo = Vec<ServiceAccessPolicyEntry>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceAccessPolicyEntry {
    #[serde(rename = "objectId")]
    pub object_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCosmosDbConfigurationInfo {
    #[serde(rename = "offerThroughput", skip_serializing_if = "Option::is_none")]
    pub offer_throughput: Option<i64>,
    #[serde(rename = "keyVaultKeyUri", skip_serializing_if = "Option::is_none")]
    pub key_vault_key_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceAuthenticationConfigurationInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(rename = "smartProxyEnabled", skip_serializing_if = "Option::is_none")]
    pub smart_proxy_enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCorsConfigurationInfo {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub origins: Vec<ServiceCorsConfigurationOriginEntry>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<ServiceCorsConfigurationHeaderEntry>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub methods: Vec<ServiceCorsConfigurationMethodEntry>,
    #[serde(rename = "maxAge", skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i64>,
    #[serde(rename = "allowCredentials", skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceExportConfigurationInfo {
    #[serde(rename = "storageAccountName", skip_serializing_if = "Option::is_none")]
    pub storage_account_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCorsConfigurationOriginEntry {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCorsConfigurationHeaderEntry {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCorsConfigurationMethodEntry {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesPatchDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServicesPropertiesUpdateParameters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    pub kind: services_resource::Kind,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<services_resource::Identity>,
}
pub mod services_resource {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "fhir")]
        Fhir,
        #[serde(rename = "fhir-Stu3")]
        FhirStu3,
        #[serde(rename = "fhir-R4")]
        FhirR4,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Identity {
        #[serde(rename = "principalId", skip_serializing)]
        pub principal_id: Option<String>,
        #[serde(rename = "tenantId", skip_serializing)]
        pub tenant_id: Option<String>,
        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        pub type_: Option<identity::Type>,
    }
    pub mod identity {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Type {
            SystemAssigned,
            None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesPropertiesUpdateParameters {
    #[serde(rename = "publicNetworkAccess", skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<services_properties_update_parameters::PublicNetworkAccess>,
}
pub mod services_properties_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpoint {
    #[serde(skip_serializing)]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
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
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceProperties {
    #[serde(rename = "groupId", skip_serializing)]
    pub group_id: Option<String>,
    #[serde(rename = "requiredMembers", skip_serializing)]
    pub required_members: Vec<String>,
    #[serde(rename = "requiredZoneNames", skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResultsDescription {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub status: Option<operation_results_description::Status>,
    #[serde(rename = "startTime", skip_serializing)]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
pub mod operation_results_description {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Canceled,
        Succeeded,
        Failed,
        Requested,
        Running,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "isDataAction", skip_serializing)]
    pub is_data_action: Option<bool>,
    #[serde(skip_serializing)]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
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
pub struct ErrorDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetailsInternal>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetailsInternal {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServicesForEdmUploadDescription {
    #[serde(flatten)]
    pub services_resource: ServicesResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServicesProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServicesForEdmUploadDescriptionListResult {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkServicesForEdmUploadDescription>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServicesForM365ComplianceCenterDescription {
    #[serde(flatten)]
    pub services_resource: ServicesResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServicesProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServicesForM365ComplianceCenterDescriptionListResult {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkServicesForM365ComplianceCenterDescription>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServicesForM365SecurityCenterDescription {
    #[serde(flatten)]
    pub services_resource: ServicesResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServicesProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServicesForM365SecurityCenterDescriptionListResult {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkServicesForM365SecurityCenterDescription>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServicesForO365ManagementActivityApiDescription {
    #[serde(flatten)]
    pub services_resource: ServicesResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServicesProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServicesForO365ManagementActivityApiDescriptionListResult {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkServicesForO365ManagementActivityApiDescription>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServicesForSccPowershellDescription {
    #[serde(flatten)]
    pub services_resource: ServicesResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServicesProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServicesForSccPowershellDescriptionListResult {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkServicesForSccPowershellDescription>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServicesForMipPolicySyncDescription {
    #[serde(flatten)]
    pub services_resource: ServicesResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServicesProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServicesForMipPolicySyncDescriptionListResult {
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkServicesForMipPolicySyncDescription>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemData {
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
pub mod system_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
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
