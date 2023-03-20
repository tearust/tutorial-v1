#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterLayer1EventRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Layer1Inbound {
    #[prost(oneof = "layer1_inbound::Msg", tags = "101, 102, 103, 104")]
    pub msg: ::core::option::Option<layer1_inbound::Msg>,
}
/// Nested message and enum types in `Layer1Inbound`.
pub mod layer1_inbound {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "101")]
        TappTopupEvent(super::TappTopupEvent),
        #[prost(message, tag = "102")]
        TappWithdrawEvent(super::TappWithdrawEvent),
        #[prost(message, tag = "103")]
        ValidatorChangedEvents(super::ValidatorChangedEvents),
        #[prost(message, tag = "104")]
        TransferCmlEvent(super::TransferCmlEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TappTopupEvent {
    #[prost(bytes = "vec", tag = "2")]
    pub token_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "5")]
    pub height: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TappWithdrawEvent {
    #[prost(bytes = "vec", tag = "1")]
    pub token_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "5")]
    pub height: u64,
    #[prost(uint32, repeated, tag = "6")]
    pub signer_indices: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorChangedEvents {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub validators: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", tag = "2")]
    pub multisig_threshold: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub height: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferCmlEvent {
    #[prost(bytes = "vec", tag = "1")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub cml_id: u64,
    #[prost(uint64, tag = "4")]
    pub height: u64,
}
