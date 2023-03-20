#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrbitGeneralRequest {
    #[prost(
        oneof = "orbit_general_request::Msg",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14"
    )]
    pub msg: ::core::option::Option<orbit_general_request::Msg>,
}
/// Nested message and enum types in `OrbitGeneralRequest`.
pub mod orbit_general_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "1")]
        IdentityRequest(super::IdentityRequest),
        #[prost(message, tag = "2")]
        PutViewsRequest(super::PutViewsRequest),
        #[prost(message, tag = "3")]
        GetViewsRequest(super::GetViewsRequest),
        #[prost(message, tag = "4")]
        SetOrbitIdRequest(super::SetOrbitIdRequest),
        #[prost(message, tag = "5")]
        AddMessageRequest(super::AddMessageRequest),
        #[prost(message, tag = "6")]
        GetMessageRequest(super::GetMessageRequest),
        #[prost(message, tag = "7")]
        DeleteMessageRequest(super::DeleteMessageRequest),
        #[prost(message, tag = "8")]
        ExtendMessageRequest(super::ExtendMessageRequest),
        #[prost(message, tag = "9")]
        NotificationAddMessageRequest(super::NotificationAddMessageRequest),
        #[prost(message, tag = "10")]
        NotificationGetMessageRequest(super::NotificationGetMessageRequest),
        #[prost(message, tag = "11")]
        InsertEntityRequest(super::InsertEntityRequest),
        #[prost(message, tag = "12")]
        ReadEntityRequest(super::ReadEntityRequest),
        #[prost(message, tag = "13")]
        DeleteEntityRequest(super::DeleteEntityRequest),
        #[prost(message, tag = "14")]
        UpdateEntityRequest(super::UpdateEntityRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneralResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityResponse {
    #[prost(message, optional, tag = "1")]
    pub response: ::core::option::Option<GeneralResponse>,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub public_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutViewsRequest {
    #[prost(uint64, tag = "1")]
    pub tapp_id: u64,
    #[prost(uint64, tag = "2")]
    pub count: u64,
    #[prost(uint64, tag = "3")]
    pub time: u64,
    #[prost(message, optional, tag = "4")]
    pub block: ::core::option::Option<Block>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutViewsResponse {
    #[prost(message, optional, tag = "1")]
    pub response: ::core::option::Option<GeneralResponse>,
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetViewsRequest {
    #[prost(uint64, tag = "1")]
    pub tapp_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetViewsResponse {
    #[prost(message, optional, tag = "1")]
    pub response: ::core::option::Option<GeneralResponse>,
    #[prost(uint64, tag = "2")]
    pub count: u64,
    #[prost(uint64, tag = "3")]
    pub time: u64,
    #[prost(message, optional, tag = "4")]
    pub block: ::core::option::Option<Block>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrbitIdRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub dbname: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrbitIdResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(uint32, tag = "1")]
    pub height: u32,
}
/// ******** BBS ********
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrbitBbsResponse {
    #[prost(message, optional, tag = "1")]
    pub response: ::core::option::Option<GeneralResponse>,
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMessageRequest {
    #[prost(uint64, tag = "1")]
    pub tapp_id: u64,
    #[prost(string, tag = "2")]
    pub dbname: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub content: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub utc: u64,
    #[prost(uint64, tag = "6")]
    pub utc_expired: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMessageRequest {
    #[prost(uint64, tag = "1")]
    pub tapp_id: u64,
    #[prost(string, tag = "2")]
    pub dbname: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub utc: u64,
    #[prost(string, tag = "4")]
    pub sender: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMessageRequest {
    #[prost(uint64, tag = "1")]
    pub tapp_id: u64,
    #[prost(string, tag = "2")]
    pub dbname: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub msg_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendMessageRequest {
    #[prost(uint64, tag = "1")]
    pub tapp_id: u64,
    #[prost(string, tag = "2")]
    pub dbname: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub msg_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub utc_expired: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationAddMessageRequest {
    #[prost(uint64, tag = "1")]
    pub tapp_id: u64,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub content: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub utc: u64,
    #[prost(uint64, tag = "6")]
    pub utc_expired: u64,
    #[prost(uint64, tag = "7")]
    pub from_tapp_id: u64,
    #[prost(string, tag = "8")]
    pub from_tapp_url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationGetMessageRequest {
    #[prost(uint64, tag = "1")]
    pub utc: u64,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertEntityRequest {
    #[prost(uint64, tag = "1")]
    pub tapp_id: u64,
    #[prost(string, tag = "2")]
    pub body_str: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub block: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadEntityRequest {
    #[prost(uint64, tag = "1")]
    pub tapp_id: u64,
    #[prost(string, repeated, tag = "2")]
    pub id_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEntityRequest {
    #[prost(uint64, tag = "1")]
    pub tapp_id: u64,
    #[prost(string, repeated, tag = "2")]
    pub id_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEntityRequest {}
