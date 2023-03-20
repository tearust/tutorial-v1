#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskReceipt {
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub network: ::core::option::Option<Network>,
    #[prost(message, optional, tag = "3")]
    pub timespan: ::core::option::Option<Timespan>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Network {
    #[prost(message, optional, tag = "2")]
    pub inbound: ::core::option::Option<Inbound>,
    #[prost(message, optional, tag = "3")]
    pub outbound: ::core::option::Option<Outbound>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Inbound {
    #[prost(uint64, tag = "1")]
    pub bytes: u64,
    #[prost(uint64, tag = "2")]
    pub unit_price: u64,
    #[prost(uint64, tag = "3")]
    pub price_coefficient: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Outbound {
    #[prost(uint64, tag = "1")]
    pub bytes: u64,
    #[prost(uint64, tag = "2")]
    pub unit_price: u64,
    #[prost(uint64, tag = "3")]
    pub price_coefficient: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timespan {
    /// milliseconds serialized by uint128
    #[prost(bytes = "vec", tag = "1")]
    pub milliseconds: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub unit_price: u64,
    #[prost(uint64, tag = "3")]
    pub price_coefficient: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageReceipt {
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub memory: ::core::option::Option<Memory>,
    #[prost(message, optional, tag = "3")]
    pub disk: ::core::option::Option<Disk>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Memory {
    #[prost(uint64, tag = "1")]
    pub bytes: u64,
    /// milliseconds serialized by uint128
    #[prost(bytes = "vec", tag = "2")]
    pub duration: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub unit_price: u64,
    #[prost(uint64, tag = "4")]
    pub price_coefficient: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Disk {
    #[prost(uint64, tag = "1")]
    pub bytes: u64,
    /// milliseconds serialized by uint128
    #[prost(bytes = "vec", tag = "2")]
    pub duration: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub unit_price: u64,
    #[prost(uint64, tag = "4")]
    pub price_coefficient: u64,
}
