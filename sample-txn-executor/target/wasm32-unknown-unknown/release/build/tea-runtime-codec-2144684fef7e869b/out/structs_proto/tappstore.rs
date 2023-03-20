#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TeaBalanceRequest {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub auth_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TeaBalanceResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub balance: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub ts: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountAssetRequest {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub auth_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountAssetResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub tea_balance: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub token_balance: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub reserved_token_balance: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenAllowanceRequest {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub token_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenAllowanceResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub balance: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignAndSendExtrinsicRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub extrinsic_call: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub token_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignAndSendExtrinsicResponse {
    #[prost(string, tag = "1")]
    pub block_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckUserSessionRequest {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub tea_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckUserSessionResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub auth_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub aes_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConsumeAccountPubkeyRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub token_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConsumeAccountPubkeyResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonSqlQueryRequest {
    #[prost(
        oneof = "common_sql_query_request::Msg",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13"
    )]
    pub msg: ::core::option::Option<common_sql_query_request::Msg>,
}
/// Nested message and enum types in `CommonSqlQueryRequest`.
pub mod common_sql_query_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "1")]
        QueryEntityListRequest(super::QueryEntityListRequest),
        #[prost(message, tag = "2")]
        QueryFavTappsRequest(super::QueryFavTappsRequest),
        #[prost(message, tag = "3")]
        QuerySeatListRequest(super::QuerySeatListRequest),
        #[prost(message, tag = "4")]
        QueryLeaderboardRequest(super::QueryLeaderboardRequest),
        #[prost(message, tag = "5")]
        QueryActiveMinerRequest(super::QueryActiveMinerRequest),
        #[prost(message, tag = "6")]
        QueryMinerEntityByTeaIdRequest(super::QueryMinerEntityByTeaIdRequest),
        #[prost(message, tag = "7")]
        QuerySeedAuctionListRequest(super::QuerySeedAuctionListRequest),
        #[prost(message, tag = "8")]
        QueryMachineInfoListRequest(super::QueryMachineInfoListRequest),
        #[prost(message, tag = "9")]
        QueryTwitterCandidateListRequest(super::QueryTwitterCandidateListRequest),
        #[prost(message, tag = "10")]
        QueryFluencerRefcodeRequest(super::QueryFluencerRefcodeRequest),
        #[prost(message, tag = "11")]
        QueryTappMetadataRequest(super::QueryTappMetadataRequest),
        #[prost(message, tag = "12")]
        QueryActiveMetadata(super::QueryActiveMetadata),
        #[prost(message, tag = "13")]
        QueryTxnGasFeeRequest(super::QueryTxnGasFeeRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEntityListRequest {
    #[prost(bool, tag = "1")]
    pub only_tapp: bool,
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub token_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTappMetadataRequest {
    #[prost(string, tag = "3")]
    pub token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFavTappsRequest {
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySeatListRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub maintainer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub deal_user: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLeaderboardRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySeedAuctionListRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMachineInfoListRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryActiveMinerRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryActiveMetadata {
    #[prost(string, optional, tag = "1")]
    pub ticker: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "2")]
    pub cml_id: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMinerEntityByTeaIdRequest {
    #[prost(string, tag = "1")]
    pub tea_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTwitterCandidateListRequest {
    #[prost(string, tag = "1")]
    pub token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFluencerRefcodeRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTxnGasFeeRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonSqlQueryResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub err: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushNotificationsInnerRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, repeated, tag = "3")]
    pub expired_heights: ::prost::alloc::vec::Vec<u32>,
    #[prost(string, tag = "4")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    pub tsid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMinerInfoRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub tea_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub at_height: u32,
    #[prost(bool, tag = "3")]
    pub find_at_height: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMiningVariableRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub tea_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNodeProfileByConnId {
    #[prost(string, tag = "1")]
    pub conn_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNodeProfileByTeaId {
    #[prost(bytes = "vec", tag = "1")]
    pub tea_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNodeProfilesByTeaIds {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub tea_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNodeProfilesByConnIds {
    #[prost(string, repeated, tag = "1")]
    pub conn_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryActiveNodes {
    #[prost(bool, tag = "2")]
    pub has_exclude: bool,
    #[prost(bytes = "vec", tag = "3")]
    pub exclude_tea_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryActiveSeats {
    #[prost(bool, tag = "2")]
    pub has_exclude: bool,
    #[prost(bytes = "vec", tag = "1")]
    pub exclude_tea_id: ::prost::alloc::vec::Vec<u8>,
}
