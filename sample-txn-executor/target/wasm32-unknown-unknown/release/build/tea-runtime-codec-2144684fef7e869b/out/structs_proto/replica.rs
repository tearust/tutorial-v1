#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateSyncMsg {
    #[prost(bytes = "vec", tag = "1")]
    pub rep_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveSyncMsg {
    #[prost(bytes = "vec", tag = "1")]
    pub sync_msg: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PopReadyTsidReq {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PopReadyTsidResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub tsid_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub txn_bytes: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReplicaCount {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearOutSyncReplica {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveTxn {
    #[prost(bytes = "vec", tag = "1")]
    pub txn_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub args: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveFollowup {
    #[prost(bytes = "vec", tag = "1")]
    pub followup: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetReplicaIdsRequest {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub replica_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoundTableMembersRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoundTableMembersResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub replica_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, repeated, tag = "2")]
    pub conn_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes = "vec", tag = "3")]
    pub update_tsid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotifyReplicaMembers {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub replica_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, repeated, tag = "2")]
    pub conn_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes = "vec", tag = "3")]
    pub update_tsid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistorySinceRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub end_tsid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistorySinceResponse {
    #[prost(message, repeated, tag = "1")]
    pub history_items: ::prost::alloc::vec::Vec<HistoryItem>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoverHistoryRequest {
    #[prost(message, repeated, tag = "1")]
    pub history_items: ::prost::alloc::vec::Vec<HistoryItem>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastHistoryRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastHistoryResponse {
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<HistoryItem>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoryItem {
    #[prost(bytes = "vec", tag = "1")]
    pub tsid: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub txn: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub args: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorsStateRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorsStateResponse {
    #[prost(message, optional, tag = "1")]
    pub validators_state: ::core::option::Option<ValidatorsState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorsState {
    #[prost(uint32, tag = "1")]
    pub desired_count: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub replicas_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub update_tsid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorMembersRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorMembersResponse {
    #[prost(message, optional, tag = "1")]
    pub validator_members: ::core::option::Option<ValidatorMembers>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorMembers {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub members: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, repeated, tag = "2")]
    pub conn_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindExecutedTxnRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub txn_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub ts: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindExecutedTxnResponse {
    #[prost(message, optional, tag = "1")]
    pub executed_txn: ::core::option::Option<ExecutedTxn>,
    #[prost(string, tag = "2")]
    pub error_msg: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutedTxn {
    #[prost(bytes = "vec", tag = "1")]
    pub txn_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub tsid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportTxnExecErrRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub txn_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub error_msg: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConsensusReplicasRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConsensusReplicasResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub replica_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExecCursorRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExecCursorResponse {
    #[prost(message, optional, tag = "1")]
    pub exec_cursor: ::core::option::Option<ExecCursor>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecCursor {
    #[prost(bytes = "vec", tag = "1")]
    pub tsid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetExecCursorRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub tsid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsetShouldLock {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetReplicaMembersRequest {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub validators: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, repeated, tag = "2")]
    pub conn_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
