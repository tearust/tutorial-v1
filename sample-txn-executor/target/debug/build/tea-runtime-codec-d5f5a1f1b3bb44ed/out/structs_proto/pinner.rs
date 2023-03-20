#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertyKeyPair {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerApproveRaRequest {
    #[prost(string, tag = "1")]
    pub peer_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub properties: ::prost::alloc::vec::Vec<PropertyKeyPair>,
    #[prost(string, tag = "3")]
    pub send_to_actor: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindPinnersRequest {
    #[prost(string, tag = "1")]
    pub deployment_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub properties: ::prost::alloc::vec::Vec<PropertyKeyPair>,
    #[prost(uint64, tag = "3")]
    pub delay_seconds: u64,
    #[prost(bytes = "vec", tag = "4")]
    pub finding_mode: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "5")]
    pub send_to_actor: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientOperationAfterVerify {
    #[prost(string, tag = "1")]
    pub peer_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub pinner_ephemeral_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub item: ::core::option::Option<ChallangeStoreItem>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChallangeStoreItem {
    #[prost(bytes = "vec", tag = "1")]
    pub state: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub properties: ::prost::alloc::vec::Vec<PropertyKeyPair>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCheckStrategy {
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<ChallangeStoreItem>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCheckStrategyResult {
    #[prost(bool, tag = "1")]
    pub verify: bool,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataUploadCompletedProcessRequest {
    #[prost(message, optional, tag = "1")]
    pub cid_code: ::core::option::Option<StringValue>,
    #[prost(message, optional, tag = "2")]
    pub cid_description: ::core::option::Option<StringValue>,
    #[prost(message, optional, tag = "3")]
    pub cid_capchecker: ::core::option::Option<StringValue>,
    #[prost(message, optional, tag = "4")]
    pub key_url_encoded: ::core::option::Option<StringValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitDataUploadRequest {
    #[prost(string, tag = "1")]
    pub deployment_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub cid_code: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub cid_description: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub cid_capchecker: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConflictListRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub deployment_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub current_items: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "4")]
    pub max_allowed: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeploymentInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub code_cid: ::core::option::Option<StringValue>,
    #[prost(message, optional, tag = "2")]
    pub description_cid: ::core::option::Option<StringValue>,
    #[prost(message, optional, tag = "3")]
    pub key1: ::core::option::Option<BytesValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesValue {
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringValue {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
