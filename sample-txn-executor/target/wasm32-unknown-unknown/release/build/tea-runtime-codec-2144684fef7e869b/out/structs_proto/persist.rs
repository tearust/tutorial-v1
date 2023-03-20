#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistRequest {
    #[prost(uint64, tag = "1")]
    pub seq_number: u64,
    #[prost(oneof = "persist_request::Msg", tags = "4, 5, 6, 7, 8, 9, 10, 11, 12, 13")]
    pub msg: ::core::option::Option<persist_request::Msg>,
}
/// Nested message and enum types in `PersistRequest`.
pub mod persist_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "4")]
        Set(super::Set),
        #[prost(message, tag = "5")]
        SetWithPrefix(super::SetWithPrefix),
        #[prost(message, tag = "6")]
        GetRequest(super::GetRequest),
        #[prost(message, tag = "7")]
        FindRequest(super::FindRequest),
        #[prost(message, tag = "8")]
        FindWithDirection(super::FindWithDirection),
        #[prost(message, tag = "9")]
        GetLastRequest(super::GetLastRequest),
        #[prost(message, tag = "10")]
        AppendDigest(super::AppendDigest),
        #[prost(message, tag = "11")]
        AppendStatements(super::AppendStatements),
        #[prost(message, tag = "12")]
        GetStatements(super::GetStatements),
        #[prost(message, tag = "13")]
        WriteFile(super::WriteFile),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistResponse {
    #[prost(uint64, tag = "1")]
    pub seq_number: u64,
    #[prost(oneof = "persist_response::Msg", tags = "4, 5, 6, 7, 8, 9")]
    pub msg: ::core::option::Option<persist_response::Msg>,
}
/// Nested message and enum types in `PersistResponse`.
pub mod persist_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "4")]
        SuccessMessage(super::SuccessMessage),
        #[prost(message, tag = "5")]
        ErrorMessage(super::ErrorMessage),
        #[prost(message, tag = "6")]
        GetResponse(super::GetResponse),
        #[prost(message, tag = "7")]
        FindResponse(super::FindResponse),
        #[prost(message, tag = "8")]
        GetLastResponse(super::GetLastResponse),
        #[prost(message, tag = "9")]
        GetStatementsResponse(super::GetStatementsResponse),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuccessMessage {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorMessage {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Set {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetWithPrefix {
    #[prost(bytes = "vec", tag = "1")]
    pub prefix: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<SingleValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleValue {
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastResponse {
    #[prost(message, optional, tag = "1")]
    pub kvp: ::core::option::Option<KeyValuePair>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendDigest {
    #[prost(bytes = "vec", tag = "1")]
    pub json_serial: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendStatements {
    #[prost(bytes = "vec", tag = "1")]
    pub statements_serial: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteFile {
    #[prost(string, tag = "1")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatements {
    #[prost(message, optional, tag = "1")]
    pub account_filter: ::core::option::Option<GetStatementsAccount>,
    #[prost(uint64, tag = "2")]
    pub max_size: u64,
    #[prost(message, optional, tag = "3")]
    pub date: ::core::option::Option<GetStatementsDatetime>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatementsAccount {
    #[prost(bytes = "vec", tag = "1")]
    pub account: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatementsDatetime {
    #[prost(int32, tag = "1")]
    pub year: i32,
    #[prost(uint32, tag = "2")]
    pub month: u32,
    #[prost(uint32, tag = "3")]
    pub day: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatementsResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub statements_serial: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub read_to_end: bool,
}
/// find with prefix and return matched results
/// return result is `FindResponse`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub prefix: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub options: ::core::option::Option<IterOptions>,
    #[prost(bool, tag = "3")]
    pub only_key: bool,
}
/// If find the given key then return sequences before/after the key
/// return result is `FindResponse`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindWithDirection {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// sequence before the specified key
    #[prost(bool, tag = "2")]
    pub before: bool,
    #[prost(message, optional, tag = "3")]
    pub options: ::core::option::Option<IterOptions>,
    #[prost(bool, tag = "4")]
    pub only_key: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindResponse {
    #[prost(message, repeated, tag = "1")]
    pub kvp_list: ::prost::alloc::vec::Vec<KeyValuePair>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValuePair {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IterOptions {
    #[prost(uint32, tag = "1")]
    pub start_index: u32,
    #[prost(uint32, tag = "2")]
    pub count: u32,
}
