#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThirdApiGeneralRequest {
    #[prost(oneof = "third_api_general_request::Msg", tags = "1, 2, 3")]
    pub msg: ::core::option::Option<third_api_general_request::Msg>,
}
/// Nested message and enum types in `ThirdApiGeneralRequest`.
pub mod third_api_general_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "1")]
        GetQueryRequest(super::GetQueryRequest),
        #[prost(message, tag = "2")]
        PostQueryRequest(super::PostQueryRequest),
        #[prost(message, tag = "3")]
        CryptPostRequest(super::CryptPostRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQueryRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub url: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub header: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostQueryRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub url: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub header: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptPostRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub url: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub header: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThirdAPiGeneralResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub data: ::prost::alloc::string::String,
}
