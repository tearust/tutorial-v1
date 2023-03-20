#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdapterServerRequest {
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(oneof = "adapter_server_request::Msg", tags = "2, 3, 4, 5, 6, 7")]
    pub msg: ::core::option::Option<adapter_server_request::Msg>,
}
/// Nested message and enum types in `AdapterServerRequest`.
pub mod adapter_server_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "2")]
        AdapterHttpRequest(super::AdapterHttpRequest),
        #[prost(message, tag = "3")]
        CheckValidRsaPubKeyRequest(super::CheckValidRsaPubKeyRequest),
        #[prost(message, tag = "4")]
        DataUploadCompletedRequest(super::DataUploadCompletedRequest),
        #[prost(message, tag = "5")]
        IpfsInboundP2pForwardRequest(super::IpfsInboundP2pForwardRequest),
        #[prost(message, tag = "6")]
        IpfsP2pForwardResponse(super::IpfsP2pForwardResponse),
        #[prost(message, tag = "7")]
        IpfsBlockGetResponse(super::IpfsBlockGetResponse),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdapterServerResponse {
    #[prost(oneof = "adapter_server_response::Msg", tags = "1, 2")]
    pub msg: ::core::option::Option<adapter_server_response::Msg>,
}
/// Nested message and enum types in `AdapterServerResponse`.
pub mod adapter_server_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "1")]
        CheckValidRsaPubKeyResponse(super::CheckValidRsaPubKeyResponse),
        #[prost(message, tag = "2")]
        IpfsInboundP2pForwardResponse(super::IpfsInboundP2pForwardResponse),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdapterClientRequest {
    #[prost(uint64, tag = "1")]
    pub seq_number: u64,
    #[prost(oneof = "adapter_client_request::Msg", tags = "2, 3, 4, 5, 6")]
    pub msg: ::core::option::Option<adapter_client_request::Msg>,
}
/// Nested message and enum types in `AdapterClientRequest`.
pub mod adapter_client_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "2")]
        IpfsBlockGetRequest(super::IpfsBlockGetRequest),
        #[prost(message, tag = "3")]
        IpfsInfoRequest(super::IpfsInfoRequest),
        #[prost(message, tag = "4")]
        IpfsP2pFrowardRequest(super::IpfsP2pFrowardRequest),
        #[prost(message, tag = "5")]
        IpfsP2pCloseRequest(super::IpfsP2pCloseRequest),
        #[prost(message, tag = "6")]
        IpfsPullCidDataRequest(super::IpfsPullCidDataRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdapterClientResponse {
    #[prost(oneof = "adapter_client_response::Msg", tags = "1")]
    pub msg: ::core::option::Option<adapter_client_response::Msg>,
}
/// Nested message and enum types in `AdapterClientResponse`.
pub mod adapter_client_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "1")]
        IpfsInfoResponse(super::IpfsInfoResponse),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpGeneralRequest {
    #[prost(uint64, tag = "1")]
    pub seq_number: u64,
    #[prost(oneof = "http_general_request::Msg", tags = "6, 7, 8")]
    pub msg: ::core::option::Option<http_general_request::Msg>,
}
/// Nested message and enum types in `HttpGeneralRequest`.
pub mod http_general_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "6")]
        HttpExecutionRequest(super::HttpExecutionRequest),
        #[prost(message, tag = "7")]
        UpgradeVersionRequest(super::UpgradeVersionRequest),
        #[prost(message, tag = "8")]
        ImportRequest(super::ImportRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpExecutionRequest {
    #[prost(string, tag = "1")]
    pub request_url: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub headers: ::prost::alloc::vec::Vec<HttpExecutionHeader>,
    #[prost(message, optional, tag = "3")]
    pub timeout: ::core::option::Option<HttpExecutionTimeout>,
    #[prost(string, tag = "4")]
    pub method: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub payload: ::core::option::Option<HttpExecutionPayload>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeVersionRequest {
    #[prost(string, tag = "1")]
    pub upgrade_type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub module_items: ::prost::alloc::vec::Vec<ModuleItem>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleItem {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportRequest {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeVersionResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub app: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "2")]
    pub modules: ::prost::alloc::vec::Vec<DownloadModule>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadModule {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub buf: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpExecutionTimeout {
    #[prost(uint64, tag = "1")]
    pub milliseconds: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpExecutionPayload {
    #[prost(bytes = "vec", tag = "1")]
    pub json_body: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpExecutionHeader {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpExecutionResponse {
    #[prost(string, tag = "1")]
    pub response_json: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpExecutionError {
    #[prost(string, tag = "1")]
    pub error_message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpfsInfoRequest {
    #[prost(string, tag = "1")]
    pub peer_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpfsInfoResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub agent_version: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub protocol_version: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpfsP2pFrowardRequest {
    #[prost(string, tag = "1")]
    pub peer_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub reply: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub p2p_general_msg: ::core::option::Option<super::p2p::GeneralMsg>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpfsP2pCloseRequest {
    #[prost(string, tag = "1")]
    pub peer_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpfsPullCidDataRequest {
    #[prost(string, tag = "1")]
    pub peer_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub reply: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "4")]
    pub pin: bool,
    #[prost(string, tag = "5")]
    pub cid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpfsBlockGetRequest {
    #[prost(string, tag = "1")]
    pub cid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub reply: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub wait_locally: bool,
    #[prost(bool, tag = "4")]
    pub return_if_not_exist: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdapterHttpRequest {
    #[prost(string, tag = "2")]
    pub action: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub actor: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckValidRsaPubKeyRequest {
    #[prost(string, tag = "1")]
    pub rsa_pub: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckValidRsaPubKeyResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub result: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataUploadCompletedRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpfsInboundP2pForwardRequest {
    #[prost(string, tag = "1")]
    pub peer_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub p2p_general_msg: ::core::option::Option<super::p2p::GeneralMsg>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpfsInboundP2pForwardResponse {
    #[prost(message, optional, tag = "1")]
    pub p2p_reply_message: ::core::option::Option<super::p2p::P2pReplyMessage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpfsP2pForwardResponse {
    #[prost(message, optional, tag = "1")]
    pub p2p_reply_message: ::core::option::Option<super::p2p::P2pReplyMessage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpfsBlockGetResponse {
    #[prost(string, tag = "1")]
    pub cid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub reply: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
