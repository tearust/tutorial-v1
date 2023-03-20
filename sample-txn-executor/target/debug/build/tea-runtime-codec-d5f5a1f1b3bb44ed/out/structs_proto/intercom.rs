#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityCountRequest {
    #[prost(uint64, tag = "1")]
    pub tapp_id: u64,
    #[prost(string, tag = "2")]
    pub uuid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityCountResponse {
    #[prost(uint64, tag = "1")]
    pub activity_count: u64,
    #[prost(uint32, tag = "2")]
    pub block_number: u32,
    #[prost(string, tag = "3")]
    pub uuid: ::prost::alloc::string::String,
}
