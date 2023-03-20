#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Libp2pRequest {
    #[prost(
        oneof = "libp2p_request::Msg",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15"
    )]
    pub msg: ::core::option::Option<libp2p_request::Msg>,
}
/// Nested message and enum types in `Libp2pRequest`.
pub mod libp2p_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "1")]
        GeneralRequest(super::GeneralRequest),
        #[prost(message, tag = "2")]
        PubMessage(super::PubMessage),
        #[prost(message, tag = "3")]
        StartUseIpAddressRequest(super::StartUseIpAddressRequest),
        #[prost(message, tag = "4")]
        StopUseIpAddressRequest(super::StopUseIpAddressRequest),
        #[prost(message, tag = "5")]
        CloseRequest(super::CloseRequest),
        #[prost(message, tag = "6")]
        PeersChangedRequest(super::PeersChangedRequest),
        #[prost(message, tag = "7")]
        SubscribeGossipTopicRequest(super::SubscribeGossipTopicRequest),
        #[prost(message, tag = "8")]
        UnsubscribeGossipTopicRequest(super::UnsubscribeGossipTopicRequest),
        #[prost(message, tag = "9")]
        ReplyRequest(super::GeneralRequest),
        #[prost(message, tag = "10")]
        Libp2pInitedRequest(super::Libp2pInitedRequest),
        #[prost(message, tag = "12")]
        ListConnectedPeers(super::ListConnectedPeers),
        #[prost(message, tag = "13")]
        AskForHandshakeRequest(super::AskForHandshakeRequest),
        #[prost(message, tag = "14")]
        HandshakeRequest(super::HandshakeRequest),
        #[prost(message, tag = "15")]
        HandshakeReply(super::HandshakeReply),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AskForHandshakeRequest {
    #[prost(string, tag = "1")]
    pub source_conn_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub target_conn_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandshakeRequest {
    #[prost(string, tag = "1")]
    pub source_conn_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub target_conn_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub ciphertext: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandshakeReply {
    #[prost(string, tag = "1")]
    pub source_conn_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub target_conn_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneralRequest {
    #[prost(string, tag = "1")]
    pub source_conn_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub target_conn_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub runtime_message: ::core::option::Option<RuntimeMessage>,
    #[prost(uint64, tag = "4")]
    pub seq_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubMessage {
    #[prost(string, tag = "1")]
    pub source_conn_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub runtime_message: ::core::option::Option<RuntimeMessage>,
    #[prost(message, optional, tag = "3")]
    pub topic: ::core::option::Option<Topic>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Topic {
    #[prost(string, tag = "1")]
    pub topic_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartUseIpAddressRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopUseIpAddressRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Libp2pInitedRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectedPeers {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeersChangedRequest {
    #[prost(string, repeated, tag = "1")]
    pub peers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPeersRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPeersResponse {
    #[prost(string, repeated, tag = "1")]
    pub peers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RandomPeersRequest {
    #[prost(uint32, tag = "1")]
    pub count: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RandomoPeersResponse {
    #[prost(string, repeated, tag = "1")]
    pub peers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "2")]
    pub insufficient_peers: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeMessage {
    #[prost(message, optional, tag = "1")]
    pub source_address: ::core::option::Option<RuntimeAddress>,
    #[prost(message, optional, tag = "2")]
    pub target_address: ::core::option::Option<RuntimeAddress>,
    #[prost(bytes = "vec", tag = "3")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeAddress {
    /// if type is actor key is actor id, if type is provider key is provider cap id
    #[prost(bytes = "vec", tag = "1")]
    pub target_key: ::prost::alloc::vec::Vec<u8>,
    /// if type is actor action is message subject in OP_DELIVER_MESSAGE, if type is provider action is custom
    /// action string about domain soket
    #[prost(string, tag = "3")]
    pub target_action: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyConnIdRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterChannelRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterChannelResponse {
    #[prost(string, tag = "1")]
    pub to_libp2p_file: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub from_libp2p_file: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionStateRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateMessageRequest {
    #[prost(string, tag = "1")]
    pub action: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub msg_b64: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub uuid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeGossipTopicRequest {
    #[prost(string, tag = "1")]
    pub topic_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsubscribeGossipTopicRequest {
    #[prost(string, tag = "2")]
    pub topic_name: ::prost::alloc::string::String,
}
