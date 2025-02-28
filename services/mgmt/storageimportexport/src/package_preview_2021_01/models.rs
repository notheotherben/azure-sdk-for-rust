#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response::Error>,
}
pub mod error_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Error {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub details: Vec<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub innererror: Option<serde_json::Value>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PutJobParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDetails {
    #[serde(rename = "storageAccountId", skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[serde(rename = "jobType", skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    #[serde(rename = "returnAddress", skip_serializing_if = "Option::is_none")]
    pub return_address: Option<ReturnAddress>,
    #[serde(rename = "returnShipping", skip_serializing_if = "Option::is_none")]
    pub return_shipping: Option<ReturnShipping>,
    #[serde(rename = "shippingInformation", skip_serializing_if = "Option::is_none")]
    pub shipping_information: Option<ShippingInformation>,
    #[serde(rename = "deliveryPackage", skip_serializing_if = "Option::is_none")]
    pub delivery_package: Option<DeliveryPackageInformation>,
    #[serde(rename = "returnPackage", skip_serializing_if = "Option::is_none")]
    pub return_package: Option<PackageInformation>,
    #[serde(rename = "diagnosticsPath", skip_serializing_if = "Option::is_none")]
    pub diagnostics_path: Option<String>,
    #[serde(rename = "logLevel", skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(rename = "backupDriveManifest", skip_serializing_if = "Option::is_none")]
    pub backup_drive_manifest: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "cancelRequested", skip_serializing_if = "Option::is_none")]
    pub cancel_requested: Option<bool>,
    #[serde(rename = "percentComplete", skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i64>,
    #[serde(rename = "incompleteBlobListUri", skip_serializing_if = "Option::is_none")]
    pub incomplete_blob_list_uri: Option<String>,
    #[serde(rename = "driveList", skip_serializing_if = "Vec::is_empty")]
    pub drive_list: Vec<DriveStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export: Option<Export>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "encryptionKey", skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKeyDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionKeyDetails {
    #[serde(rename = "kekType", skip_serializing_if = "Option::is_none")]
    pub kek_type: Option<encryption_key_details::KekType>,
    #[serde(rename = "kekUrl", skip_serializing_if = "Option::is_none")]
    pub kek_url: Option<String>,
    #[serde(rename = "kekVaultResourceID", skip_serializing_if = "Option::is_none")]
    pub kek_vault_resource_id: Option<String>,
}
pub mod encryption_key_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KekType {
        MicrosoftManaged,
        CustomerManaged,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityDetails {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity_details::Type>,
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
}
pub mod identity_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        None,
        SystemAssigned,
        UserAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateJobParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<update_job_parameters::Properties>,
}
pub mod update_job_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "cancelRequested", skip_serializing_if = "Option::is_none")]
        pub cancel_requested: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[serde(rename = "returnAddress", skip_serializing_if = "Option::is_none")]
        pub return_address: Option<ReturnAddress>,
        #[serde(rename = "returnShipping", skip_serializing_if = "Option::is_none")]
        pub return_shipping: Option<ReturnShipping>,
        #[serde(rename = "deliveryPackage", skip_serializing_if = "Option::is_none")]
        pub delivery_package: Option<DeliveryPackageInformation>,
        #[serde(rename = "logLevel", skip_serializing_if = "Option::is_none")]
        pub log_level: Option<String>,
        #[serde(rename = "backupDriveManifest", skip_serializing_if = "Option::is_none")]
        pub backup_drive_manifest: Option<bool>,
        #[serde(rename = "driveList", skip_serializing_if = "Vec::is_empty")]
        pub drive_list: Vec<DriveStatus>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListJobsResponse {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobResponse {
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    pub name: String,
    pub display: operation::Display,
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
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<location::Properties>,
}
pub mod location {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "recipientName", skip_serializing_if = "Option::is_none")]
        pub recipient_name: Option<String>,
        #[serde(rename = "streetAddress1", skip_serializing_if = "Option::is_none")]
        pub street_address1: Option<String>,
        #[serde(rename = "streetAddress2", skip_serializing_if = "Option::is_none")]
        pub street_address2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        #[serde(rename = "stateOrProvince", skip_serializing_if = "Option::is_none")]
        pub state_or_province: Option<String>,
        #[serde(rename = "postalCode", skip_serializing_if = "Option::is_none")]
        pub postal_code: Option<String>,
        #[serde(rename = "countryOrRegion", skip_serializing_if = "Option::is_none")]
        pub country_or_region: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
        #[serde(rename = "additionalShippingInformation", skip_serializing_if = "Option::is_none")]
        pub additional_shipping_information: Option<String>,
        #[serde(rename = "supportedCarriers", skip_serializing_if = "Vec::is_empty")]
        pub supported_carriers: Vec<String>,
        #[serde(rename = "alternateLocations", skip_serializing_if = "Vec::is_empty")]
        pub alternate_locations: Vec<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReturnAddress {
    #[serde(rename = "recipientName")]
    pub recipient_name: String,
    #[serde(rename = "streetAddress1")]
    pub street_address1: String,
    #[serde(rename = "streetAddress2", skip_serializing_if = "Option::is_none")]
    pub street_address2: Option<String>,
    pub city: String,
    #[serde(rename = "stateOrProvince", skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    #[serde(rename = "countryOrRegion")]
    pub country_or_region: String,
    pub phone: String,
    pub email: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReturnShipping {
    #[serde(rename = "carrierName")]
    pub carrier_name: String,
    #[serde(rename = "carrierAccountNumber")]
    pub carrier_account_number: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShippingInformation {
    #[serde(rename = "recipientName", skip_serializing_if = "Option::is_none")]
    pub recipient_name: Option<String>,
    #[serde(rename = "streetAddress1", skip_serializing_if = "Option::is_none")]
    pub street_address1: Option<String>,
    #[serde(rename = "streetAddress2", skip_serializing_if = "Option::is_none")]
    pub street_address2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "stateOrProvince", skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,
    #[serde(rename = "postalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(rename = "countryOrRegion", skip_serializing_if = "Option::is_none")]
    pub country_or_region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "additionalInformation", skip_serializing)]
    pub additional_information: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageInformation {
    #[serde(rename = "carrierName")]
    pub carrier_name: String,
    #[serde(rename = "trackingNumber")]
    pub tracking_number: String,
    #[serde(rename = "driveCount")]
    pub drive_count: i64,
    #[serde(rename = "shipDate")]
    pub ship_date: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryPackageInformation {
    #[serde(rename = "carrierName")]
    pub carrier_name: String,
    #[serde(rename = "trackingNumber")]
    pub tracking_number: String,
    #[serde(rename = "driveCount", skip_serializing_if = "Option::is_none")]
    pub drive_count: Option<i64>,
    #[serde(rename = "shipDate", skip_serializing_if = "Option::is_none")]
    pub ship_date: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DriveStatus {
    #[serde(rename = "driveId", skip_serializing_if = "Option::is_none")]
    pub drive_id: Option<String>,
    #[serde(rename = "bitLockerKey", skip_serializing_if = "Option::is_none")]
    pub bit_locker_key: Option<String>,
    #[serde(rename = "manifestFile", skip_serializing_if = "Option::is_none")]
    pub manifest_file: Option<String>,
    #[serde(rename = "manifestHash", skip_serializing_if = "Option::is_none")]
    pub manifest_hash: Option<String>,
    #[serde(rename = "driveHeaderHash", skip_serializing_if = "Option::is_none")]
    pub drive_header_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<drive_status::State>,
    #[serde(rename = "copyStatus", skip_serializing_if = "Option::is_none")]
    pub copy_status: Option<String>,
    #[serde(rename = "percentComplete", skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i64>,
    #[serde(rename = "verboseLogUri", skip_serializing_if = "Option::is_none")]
    pub verbose_log_uri: Option<String>,
    #[serde(rename = "errorLogUri", skip_serializing_if = "Option::is_none")]
    pub error_log_uri: Option<String>,
    #[serde(rename = "manifestUri", skip_serializing_if = "Option::is_none")]
    pub manifest_uri: Option<String>,
    #[serde(rename = "bytesSucceeded", skip_serializing_if = "Option::is_none")]
    pub bytes_succeeded: Option<i64>,
}
pub mod drive_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Specified,
        Received,
        NeverReceived,
        Transferring,
        Completed,
        CompletedMoreInfo,
        ShippedBack,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Export {
    #[serde(rename = "blobList", skip_serializing_if = "Option::is_none")]
    pub blob_list: Option<export::BlobList>,
    #[serde(rename = "blobListBlobPath", skip_serializing_if = "Option::is_none")]
    pub blob_list_blob_path: Option<String>,
}
pub mod export {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct BlobList {
        #[serde(rename = "blobPath", skip_serializing_if = "Vec::is_empty")]
        pub blob_path: Vec<String>,
        #[serde(rename = "blobPathPrefix", skip_serializing_if = "Vec::is_empty")]
        pub blob_path_prefix: Vec<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationsResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Location>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetBitLockerKeysResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DriveBitLockerKey>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DriveBitLockerKey {
    #[serde(rename = "bitLockerKey", skip_serializing_if = "Option::is_none")]
    pub bit_locker_key: Option<String>,
    #[serde(rename = "driveId", skip_serializing_if = "Option::is_none")]
    pub drive_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListOperationsResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
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
