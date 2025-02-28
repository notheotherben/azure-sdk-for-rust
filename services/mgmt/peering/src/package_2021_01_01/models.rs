#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CdnPeeringPrefixListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CdnPeeringPrefix>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CdnPeeringPrefix {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CdnPeeringPrefixProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CdnPeeringPrefixProperties {
    #[serde(skip_serializing)]
    pub prefix: Option<String>,
    #[serde(rename = "azureRegion", skip_serializing)]
    pub azure_region: Option<String>,
    #[serde(rename = "azureService", skip_serializing)]
    pub azure_service: Option<String>,
    #[serde(rename = "isPrimaryRegion", skip_serializing)]
    pub is_primary_region: Option<bool>,
    #[serde(rename = "bgpCommunity", skip_serializing)]
    pub bgp_community: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckServiceProviderAvailabilityInput {
    #[serde(rename = "peeringServiceLocation", skip_serializing_if = "Option::is_none")]
    pub peering_service_location: Option<String>,
    #[serde(rename = "peeringServiceProvider", skip_serializing_if = "Option::is_none")]
    pub peering_service_provider: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Peering>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Peering {
    #[serde(flatten)]
    pub resource: Resource,
    pub sku: PeeringSku,
    pub kind: peering::Kind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringProperties>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
pub mod peering {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Direct,
        Exchange,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringSku {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<peering_sku::Tier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<peering_sku::Family>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<peering_sku::Size>,
}
pub mod peering_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Basic,
        Premium,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Family {
        Direct,
        Exchange,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Size {
        Free,
        Metered,
        Unlimited,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct: Option<PeeringPropertiesDirect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<PeeringPropertiesExchange>,
    #[serde(rename = "peeringLocation", skip_serializing_if = "Option::is_none")]
    pub peering_location: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<peering_properties::ProvisioningState>,
}
pub mod peering_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Updating,
        Deleting,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringPropertiesDirect {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub connections: Vec<DirectConnection>,
    #[serde(rename = "useForPeeringService", skip_serializing)]
    pub use_for_peering_service: Option<bool>,
    #[serde(rename = "peerAsn", skip_serializing_if = "Option::is_none")]
    pub peer_asn: Option<SubResource>,
    #[serde(rename = "directPeeringType", skip_serializing_if = "Option::is_none")]
    pub direct_peering_type: Option<peering_properties_direct::DirectPeeringType>,
}
pub mod peering_properties_direct {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DirectPeeringType {
        Edge,
        Transit,
        Cdn,
        Internal,
        Ix,
        IxRs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringPropertiesExchange {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub connections: Vec<ExchangeConnection>,
    #[serde(rename = "peerAsn", skip_serializing_if = "Option::is_none")]
    pub peer_asn: Option<SubResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectConnection {
    #[serde(rename = "bandwidthInMbps", skip_serializing_if = "Option::is_none")]
    pub bandwidth_in_mbps: Option<i32>,
    #[serde(rename = "provisionedBandwidthInMbps", skip_serializing)]
    pub provisioned_bandwidth_in_mbps: Option<i32>,
    #[serde(rename = "sessionAddressProvider", skip_serializing_if = "Option::is_none")]
    pub session_address_provider: Option<direct_connection::SessionAddressProvider>,
    #[serde(rename = "useForPeeringService", skip_serializing_if = "Option::is_none")]
    pub use_for_peering_service: Option<bool>,
    #[serde(rename = "microsoftTrackingId", skip_serializing)]
    pub microsoft_tracking_id: Option<String>,
    #[serde(rename = "peeringDBFacilityId", skip_serializing_if = "Option::is_none")]
    pub peering_db_facility_id: Option<i32>,
    #[serde(rename = "connectionState", skip_serializing)]
    pub connection_state: Option<direct_connection::ConnectionState>,
    #[serde(rename = "bgpSession", skip_serializing_if = "Option::is_none")]
    pub bgp_session: Option<BgpSession>,
    #[serde(rename = "connectionIdentifier", skip_serializing_if = "Option::is_none")]
    pub connection_identifier: Option<String>,
    #[serde(rename = "errorMessage", skip_serializing)]
    pub error_message: Option<String>,
}
pub mod direct_connection {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SessionAddressProvider {
        Microsoft,
        Peer,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ConnectionState {
        None,
        PendingApproval,
        Approved,
        ProvisioningStarted,
        ProvisioningFailed,
        ProvisioningCompleted,
        Validating,
        Active,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExchangeConnection {
    #[serde(rename = "peeringDBFacilityId", skip_serializing_if = "Option::is_none")]
    pub peering_db_facility_id: Option<i32>,
    #[serde(rename = "connectionState", skip_serializing)]
    pub connection_state: Option<exchange_connection::ConnectionState>,
    #[serde(rename = "bgpSession", skip_serializing_if = "Option::is_none")]
    pub bgp_session: Option<BgpSession>,
    #[serde(rename = "connectionIdentifier", skip_serializing_if = "Option::is_none")]
    pub connection_identifier: Option<String>,
    #[serde(rename = "errorMessage", skip_serializing)]
    pub error_message: Option<String>,
}
pub mod exchange_connection {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ConnectionState {
        None,
        PendingApproval,
        Approved,
        ProvisioningStarted,
        ProvisioningFailed,
        ProvisioningCompleted,
        Validating,
        Active,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BgpSession {
    #[serde(rename = "sessionPrefixV4", skip_serializing_if = "Option::is_none")]
    pub session_prefix_v4: Option<String>,
    #[serde(rename = "sessionPrefixV6", skip_serializing_if = "Option::is_none")]
    pub session_prefix_v6: Option<String>,
    #[serde(rename = "microsoftSessionIPv4Address", skip_serializing_if = "Option::is_none")]
    pub microsoft_session_i_pv4_address: Option<String>,
    #[serde(rename = "microsoftSessionIPv6Address", skip_serializing_if = "Option::is_none")]
    pub microsoft_session_i_pv6_address: Option<String>,
    #[serde(rename = "peerSessionIPv4Address", skip_serializing_if = "Option::is_none")]
    pub peer_session_i_pv4_address: Option<String>,
    #[serde(rename = "peerSessionIPv6Address", skip_serializing_if = "Option::is_none")]
    pub peer_session_i_pv6_address: Option<String>,
    #[serde(rename = "sessionStateV4", skip_serializing)]
    pub session_state_v4: Option<bgp_session::SessionStateV4>,
    #[serde(rename = "sessionStateV6", skip_serializing)]
    pub session_state_v6: Option<bgp_session::SessionStateV6>,
    #[serde(rename = "maxPrefixesAdvertisedV4", skip_serializing_if = "Option::is_none")]
    pub max_prefixes_advertised_v4: Option<i32>,
    #[serde(rename = "maxPrefixesAdvertisedV6", skip_serializing_if = "Option::is_none")]
    pub max_prefixes_advertised_v6: Option<i32>,
    #[serde(rename = "md5AuthenticationKey", skip_serializing_if = "Option::is_none")]
    pub md5_authentication_key: Option<String>,
}
pub mod bgp_session {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SessionStateV4 {
        None,
        Idle,
        Connect,
        Active,
        OpenSent,
        OpenConfirm,
        OpenReceived,
        Established,
        PendingAdd,
        PendingUpdate,
        PendingRemove,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SessionStateV6 {
        None,
        Idle,
        Connect,
        Active,
        OpenSent,
        OpenConfirm,
        OpenReceived,
        Established,
        PendingAdd,
        PendingUpdate,
        PendingRemove,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
    #[serde(rename = "isDataAction", skip_serializing)]
    pub is_data_action: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplayInfo {
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
pub struct PeerAsn {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeerAsnProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeerAsnProperties {
    #[serde(rename = "peerAsn", skip_serializing_if = "Option::is_none")]
    pub peer_asn: Option<i32>,
    #[serde(rename = "peerContactDetail", skip_serializing_if = "Vec::is_empty")]
    pub peer_contact_detail: Vec<ContactDetail>,
    #[serde(rename = "peerName", skip_serializing_if = "Option::is_none")]
    pub peer_name: Option<String>,
    #[serde(rename = "validationState", skip_serializing_if = "Option::is_none")]
    pub validation_state: Option<peer_asn_properties::ValidationState>,
    #[serde(rename = "errorMessage", skip_serializing)]
    pub error_message: Option<String>,
}
pub mod peer_asn_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ValidationState {
        None,
        Pending,
        Approved,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<contact_detail::Role>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
pub mod contact_detail {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Role {
        Noc,
        Policy,
        Technical,
        Service,
        Escalation,
        Other,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeerAsnListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeerAsn>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringLocationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringLocation>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringLocation {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<peering_location::Kind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringLocationProperties>,
}
pub mod peering_location {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Direct,
        Exchange,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringLocationProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct: Option<PeeringLocationPropertiesDirect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<PeeringLocationPropertiesExchange>,
    #[serde(rename = "peeringLocation", skip_serializing_if = "Option::is_none")]
    pub peering_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "azureRegion", skip_serializing_if = "Option::is_none")]
    pub azure_region: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringLocationPropertiesDirect {
    #[serde(rename = "peeringFacilities", skip_serializing_if = "Vec::is_empty")]
    pub peering_facilities: Vec<DirectPeeringFacility>,
    #[serde(rename = "bandwidthOffers", skip_serializing_if = "Vec::is_empty")]
    pub bandwidth_offers: Vec<PeeringBandwidthOffer>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringLocationPropertiesExchange {
    #[serde(rename = "peeringFacilities", skip_serializing_if = "Vec::is_empty")]
    pub peering_facilities: Vec<ExchangePeeringFacility>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectPeeringFacility {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "directPeeringType", skip_serializing_if = "Option::is_none")]
    pub direct_peering_type: Option<direct_peering_facility::DirectPeeringType>,
    #[serde(rename = "peeringDBFacilityId", skip_serializing_if = "Option::is_none")]
    pub peering_db_facility_id: Option<i32>,
    #[serde(rename = "peeringDBFacilityLink", skip_serializing_if = "Option::is_none")]
    pub peering_db_facility_link: Option<String>,
}
pub mod direct_peering_facility {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DirectPeeringType {
        Edge,
        Transit,
        Cdn,
        Internal,
        Ix,
        IxRs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringBandwidthOffer {
    #[serde(rename = "offerName", skip_serializing_if = "Option::is_none")]
    pub offer_name: Option<String>,
    #[serde(rename = "valueInMbps", skip_serializing_if = "Option::is_none")]
    pub value_in_mbps: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExchangePeeringFacility {
    #[serde(rename = "exchangeName", skip_serializing_if = "Option::is_none")]
    pub exchange_name: Option<String>,
    #[serde(rename = "bandwidthInMbps", skip_serializing_if = "Option::is_none")]
    pub bandwidth_in_mbps: Option<i32>,
    #[serde(rename = "microsoftIPv4Address", skip_serializing_if = "Option::is_none")]
    pub microsoft_i_pv4_address: Option<String>,
    #[serde(rename = "microsoftIPv6Address", skip_serializing_if = "Option::is_none")]
    pub microsoft_i_pv6_address: Option<String>,
    #[serde(rename = "facilityIPv4Prefix", skip_serializing_if = "Option::is_none")]
    pub facility_i_pv4_prefix: Option<String>,
    #[serde(rename = "facilityIPv6Prefix", skip_serializing_if = "Option::is_none")]
    pub facility_i_pv6_prefix: Option<String>,
    #[serde(rename = "peeringDBFacilityId", skip_serializing_if = "Option::is_none")]
    pub peering_db_facility_id: Option<i32>,
    #[serde(rename = "peeringDBFacilityLink", skip_serializing_if = "Option::is_none")]
    pub peering_db_facility_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringRegisteredAsn {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringRegisteredAsnProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringRegisteredAsnProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i32>,
    #[serde(rename = "peeringServicePrefixKey", skip_serializing)]
    pub peering_service_prefix_key: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<peering_registered_asn_properties::ProvisioningState>,
}
pub mod peering_registered_asn_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Updating,
        Deleting,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringRegisteredAsnListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringRegisteredAsn>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringRegisteredPrefix {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringRegisteredPrefixProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringRegisteredPrefixProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "prefixValidationState", skip_serializing)]
    pub prefix_validation_state: Option<peering_registered_prefix_properties::PrefixValidationState>,
    #[serde(rename = "peeringServicePrefixKey", skip_serializing)]
    pub peering_service_prefix_key: Option<String>,
    #[serde(rename = "errorMessage", skip_serializing)]
    pub error_message: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<peering_registered_prefix_properties::ProvisioningState>,
}
pub mod peering_registered_prefix_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrefixValidationState {
        None,
        Invalid,
        Verified,
        Failed,
        Pending,
        Warning,
        Unknown,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Updating,
        Deleting,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringRegisteredPrefixListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringRegisteredPrefix>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceTags {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringReceivedRouteListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringReceivedRoute>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringReceivedRoute {
    #[serde(skip_serializing)]
    pub prefix: Option<String>,
    #[serde(rename = "nextHop", skip_serializing)]
    pub next_hop: Option<String>,
    #[serde(rename = "asPath", skip_serializing)]
    pub as_path: Option<String>,
    #[serde(rename = "originAsValidationState", skip_serializing)]
    pub origin_as_validation_state: Option<String>,
    #[serde(rename = "rpkiValidationState", skip_serializing)]
    pub rpki_validation_state: Option<String>,
    #[serde(rename = "trustAnchor", skip_serializing)]
    pub trust_anchor: Option<String>,
    #[serde(rename = "receivedTimestamp", skip_serializing)]
    pub received_timestamp: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringServiceCountryListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringServiceCountry>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringServiceCountry {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringServiceLocationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringServiceLocation>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringServiceLocation {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringServiceLocationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringServiceLocationProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "azureRegion", skip_serializing_if = "Option::is_none")]
    pub azure_region: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringServicePrefix {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringServicePrefixProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringServicePrefixProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "prefixValidationState", skip_serializing)]
    pub prefix_validation_state: Option<peering_service_prefix_properties::PrefixValidationState>,
    #[serde(rename = "learnedType", skip_serializing)]
    pub learned_type: Option<peering_service_prefix_properties::LearnedType>,
    #[serde(rename = "errorMessage", skip_serializing)]
    pub error_message: Option<String>,
    #[serde(skip_serializing)]
    pub events: Vec<PeeringServicePrefixEvent>,
    #[serde(rename = "peeringServicePrefixKey", skip_serializing_if = "Option::is_none")]
    pub peering_service_prefix_key: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<peering_service_prefix_properties::ProvisioningState>,
}
pub mod peering_service_prefix_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrefixValidationState {
        None,
        Invalid,
        Verified,
        Failed,
        Pending,
        Warning,
        Unknown,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LearnedType {
        None,
        ViaServiceProvider,
        ViaSession,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Updating,
        Deleting,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringServicePrefixEvent {
    #[serde(rename = "eventTimestamp", skip_serializing)]
    pub event_timestamp: Option<String>,
    #[serde(rename = "eventType", skip_serializing)]
    pub event_type: Option<String>,
    #[serde(rename = "eventSummary", skip_serializing)]
    pub event_summary: Option<String>,
    #[serde(rename = "eventLevel", skip_serializing)]
    pub event_level: Option<String>,
    #[serde(rename = "eventDescription", skip_serializing)]
    pub event_description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringServicePrefixListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringServicePrefix>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringServiceProviderListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringServiceProvider>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringServiceProvider {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringServiceProviderProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringServiceProviderProperties {
    #[serde(rename = "serviceProviderName", skip_serializing_if = "Option::is_none")]
    pub service_provider_name: Option<String>,
    #[serde(rename = "peeringLocations", skip_serializing_if = "Vec::is_empty")]
    pub peering_locations: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringService {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<PeeringServiceSku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PeeringServiceProperties>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringServiceSku {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringServiceProperties {
    #[serde(rename = "peeringServiceLocation", skip_serializing_if = "Option::is_none")]
    pub peering_service_location: Option<String>,
    #[serde(rename = "peeringServiceProvider", skip_serializing_if = "Option::is_none")]
    pub peering_service_provider: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<peering_service_properties::ProvisioningState>,
    #[serde(rename = "providerPrimaryPeeringLocation", skip_serializing_if = "Option::is_none")]
    pub provider_primary_peering_location: Option<String>,
    #[serde(rename = "providerBackupPeeringLocation", skip_serializing_if = "Option::is_none")]
    pub provider_backup_peering_location: Option<String>,
}
pub mod peering_service_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Updating,
        Deleting,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeeringServiceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PeeringService>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
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
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
