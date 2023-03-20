#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelOutboundMessage {
    #[prost(string, tag = "1")]
    pub provider_name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub peer_id: u64,
    #[prost(message, optional, tag = "3")]
    pub message: ::core::option::Option<ChannelMessages>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelInboundMessage {
    #[prost(string, tag = "1")]
    pub provider_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub message: ::core::option::Option<ChannelMessages>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcpWrapperMessage {
    #[prost(string, tag = "1")]
    pub provider_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub message: ::core::option::Option<ChannelMessages>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMessages {
    #[prost(oneof = "channel_messages::Msg", tags = "1, 2, 5, 6")]
    pub msg: ::core::option::Option<channel_messages::Msg>,
}
/// Nested message and enum types in `ChannelMessages`.
pub mod channel_messages {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "1")]
        NormalMessage(super::NormalMessage),
        #[prost(message, tag = "2")]
        ProposalMessage(super::ProposalMessage),
        #[prost(message, tag = "5")]
        DefaultPeerInit(super::DefaultPeerInit),
        #[prost(message, tag = "6")]
        ProposalCallback(super::ProposalCallback),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValueRequest {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub index: u32,
    #[prost(string, tag = "3")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub get_all: bool,
    #[prost(bool, tag = "5")]
    pub get_by_prefix: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValueResponse {
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<GetSingleValue>,
    #[prost(message, optional, tag = "2")]
    pub values: ::core::option::Option<GetMultipleValues>,
    #[prost(string, tag = "3")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSingleValue {
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMultipleValues {
    #[prost(string, repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetValueRequest {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "3")]
    pub index: u32,
    #[prost(string, tag = "4")]
    pub uuid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetValueResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteValueRequest {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub index: u32,
    #[prost(string, tag = "3")]
    pub uuid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteValueResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalMessage {
    /// serialize from `Message` object in raft-rs
    #[prost(bytes = "vec", tag = "1")]
    pub message: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "2")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalMessage {
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub normal: ::core::option::Option<NormalProposal>,
    #[prost(message, optional, tag = "3")]
    pub conf_change: ::core::option::Option<ConfChangeProposal>,
    #[prost(message, optional, tag = "4")]
    pub transfer_leader: ::core::option::Option<TransferLeaderProposal>,
    #[prost(uint64, tag = "5")]
    pub source_peer_id: u64,
    #[prost(message, optional, tag = "6")]
    pub delete: ::core::option::Option<DeleteProposal>,
    #[prost(int64, tag = "7")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalCallback {
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalProposal {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "3")]
    pub index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProposal {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfChangeProposal {
    /// serialize from `ConfChange` object in raft-rs
    #[prost(bytes = "vec", tag = "1")]
    pub message: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferLeaderProposal {
    #[prost(uint64, tag = "1")]
    pub peer_id: u64,
}
/// raft inner messages about proprosals
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InnerMessages {
    #[prost(oneof = "inner_messages::Msg", tags = "1, 2")]
    pub msg: ::core::option::Option<inner_messages::Msg>,
}
/// Nested message and enum types in `InnerMessages`.
pub mod inner_messages {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "1")]
        InnerChangeStorageMessage(super::InnerChangeStorageMessage),
        #[prost(message, tag = "2")]
        InnerDeleteStorageMessage(super::InnerDeleteStorageMessage),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InnerChangeStorageMessage {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "3")]
    pub index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InnerDeleteStorageMessage {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerNode {
    #[prost(uint64, tag = "1")]
    pub peer_id: u64,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub port: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DefaultPeerInit {
    #[prost(uint64, tag = "1")]
    pub peer_id: u64,
    #[prost(bool, tag = "2")]
    pub is_leader: bool,
}
