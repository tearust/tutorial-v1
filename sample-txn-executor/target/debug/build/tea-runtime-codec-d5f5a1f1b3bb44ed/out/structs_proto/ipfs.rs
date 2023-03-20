#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpfsTestRequest {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubsubPeersResponse {
    #[prost(string, tag = "1")]
    pub peers: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockGetRequest {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockPutRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockPutResponse {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub size: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindProvidersResponseItem {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub addrs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindProvidersContext {
    #[prost(string, tag = "1")]
    pub nats_subject: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub retry_remain_times: u32,
    #[prost(uint64, tag = "3")]
    pub delay_secs: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindProvidersRequest {
    #[prost(string, tag = "1")]
    pub deployment_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub context: ::core::option::Option<FindProvidersContext>,
    #[prost(string, tag = "4")]
    pub send_to_actor: ::prost::alloc::string::String,
    #[prost(enumeration = "FindingMode", tag = "5")]
    pub finding_mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindProvidersResponse {
    #[prost(string, tag = "1")]
    pub deployment_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<FindProvidersResponseItem>,
    #[prost(string, tag = "4")]
    pub send_to_actor: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub attachment: ::core::option::Option<AttachmentData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindProviderInvokeError {
    #[prost(message, optional, tag = "1")]
    pub request: ::core::option::Option<FindProvidersRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttachmentData {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpfsOperationRequest {
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub actor: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub operation: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FindingMode {
    AsMuchAsPossible = 0,
    FirstComeThenDone = 1,
}
impl FindingMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FindingMode::AsMuchAsPossible => "AsMuchAsPossible",
            FindingMode::FirstComeThenDone => "FirstComeThenDone",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AsMuchAsPossible" => Some(Self::AsMuchAsPossible),
            "FirstComeThenDone" => Some(Self::FirstComeThenDone),
            _ => None,
        }
    }
}
