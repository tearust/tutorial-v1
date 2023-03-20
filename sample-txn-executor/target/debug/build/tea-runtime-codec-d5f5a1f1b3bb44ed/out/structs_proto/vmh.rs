#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderOperationRequest {
    #[prost(string, tag = "1")]
    pub actor: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub operation: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterHttpDispatcherRequest {
    #[prost(string, repeated, tag = "1")]
    pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
