#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneralMsg {
    #[prost(string, tag = "99")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(
        oneof = "general_msg::Msg",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26"
    )]
    pub msg: ::core::option::Option<general_msg::Msg>,
}
/// Nested message and enum types in `GeneralMsg`.
pub mod general_msg {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "1")]
        Error(super::ResponseError),
        #[prost(message, tag = "2")]
        PeerApproveRequest(super::PeerApprovePinnerRequest),
        #[prost(message, tag = "3")]
        PeerApproveResponse(super::PeerApprovePinnerResponse),
        #[prost(message, tag = "4")]
        ApplyPinnerRequest(super::ApplyToBePinnerRequest),
        #[prost(message, tag = "5")]
        ApplyPinnerResponse(super::ApplyToBePinnerResponse),
        #[prost(message, tag = "6")]
        ApplyToExecuteTaskRequest(super::ApplyToExecuteTaskRequest),
        #[prost(message, tag = "7")]
        WinnerExecutorCert(super::WinnerExecutorCert),
        #[prost(message, tag = "8")]
        ErrandExecutionRequest(super::ErrandExecutionRequest),
        #[prost(message, tag = "9")]
        ErrandExecutionFailedResponse(super::ErrandExecutionFailedResponse),
        #[prost(message, tag = "10")]
        ErrandAdhocCodeCapCheckerRequest(super::ErrandAdhocCodeCapCheckerRequest),
        #[prost(message, tag = "11")]
        ErrandAdhocCodeCapCheckerResponse(super::ErrandAdhocCodeCapCheckerResponse),
        #[prost(message, tag = "12")]
        ErrandExecutionSucceededResponse(super::ErrandExecutionSucceededResponse),
        #[prost(message, tag = "13")]
        TaskKeyGenerationApplyRequst(super::TaskKeyGenerationApplyRequst),
        #[prost(message, tag = "14")]
        TaskExecutionRequest(super::TaskExecutionRequest),
        #[prost(message, tag = "15")]
        TaskExecutionResponse(super::TaskExecutionResponse),
        #[prost(message, tag = "16")]
        TaskPinnerKeySliceRequest(super::TaskPinnerKeySliceRequest),
        #[prost(message, tag = "17")]
        TaskPinnerKeySliceResponse(super::TaskPinnerKeySliceResponse),
        #[prost(message, tag = "18")]
        PeerApproveRaRequest(super::PeerApproveRaRequest),
        #[prost(message, tag = "19")]
        PeerApproveRaResponse(super::PeerApproveRaResponse),
        #[prost(message, tag = "20")]
        TaskSignWithKeySlicesRequst(super::TaskSignWithKeySlicesRequst),
        #[prost(message, tag = "21")]
        TaskSignWithKeySlicesResponse(super::TaskSignWithKeySlicesResponse),
        #[prost(message, tag = "22")]
        TaskSignGetPinnerKeySliceRequest(super::TaskSignGetPinnerKeySliceRequest),
        #[prost(message, tag = "23")]
        TaskSignGetPinnerKeySliceResponse(super::TaskSignGetPinnerKeySliceResponse),
        #[prost(message, tag = "24")]
        TaskCommitSignResultRequest(super::TaskCommitSignResultRequest),
        #[prost(message, tag = "25")]
        KeyGenerationCandidateRequest(super::KeyGenerationCandidateRequest),
        #[prost(message, tag = "26")]
        SignCandidateRequest(super::SignCandidateRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseError {
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
}
/// this message is used when Find Provider found another
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerApprovePinnerRequest {
    /// peer claimed a provider, we need to ask him and verify if he actually is
    ///
    /// this is cid of pinner_key's pub key
    #[prost(string, tag = "1")]
    pub deployment_id: ::prost::alloc::string::String,
    /// this is an random number for verify
    #[prost(uint32, tag = "2")]
    pub nonce: u32,
    /// this is the uuid of pinner store item
    #[prost(string, tag = "3")]
    pub uuid: ::prost::alloc::string::String,
    /// this is the properties passing to pinner
    #[prost(bytes = "vec", tag = "4")]
    pub properties: ::prost::alloc::vec::Vec<u8>,
    /// this is used in remote attestation
    #[prost(bytes = "vec", tag = "5")]
    pub ephemeral_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "6")]
    pub send_to_actor: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerApproveRaRequest {
    /// this is the uuid of pinner store item
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
    /// this is the properties passing to pinner
    #[prost(bytes = "vec", tag = "2")]
    pub properties: ::prost::alloc::vec::Vec<u8>,
    /// this is used in remote attestation
    #[prost(bytes = "vec", tag = "3")]
    pub ephemeral_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub send_to_actor: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerApproveRaResponse {
    /// this is the uuid of pinner store item
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
    /// this is used in remote attestation
    #[prost(bytes = "vec", tag = "2")]
    pub ephemeral_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub send_to_actor: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerApprovePinnerResponse {
    /// this is cid of pinner_key's pub key
    #[prost(string, tag = "1")]
    pub deployment_id: ::prost::alloc::string::String,
    /// The Ed25519 sig for nonce
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// this is the uuid of pinner store item
    #[prost(string, tag = "3")]
    pub uuid: ::prost::alloc::string::String,
    /// The Ehpemeral Id of the pinner
    #[prost(bytes = "vec", tag = "4")]
    pub pinner_ephemeral_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "5")]
    pub send_to_actor: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyToBePinnerRequest {
    /// this is cid of pinner_key's pub key
    #[prost(string, tag = "1")]
    pub deployment_id: ::prost::alloc::string::String,
    /// Delegate will use this RSA Pub Key to encrypt the key1 when sending to Executor. Format is tpm_protp::RsaKeyPairPemPcsk1.publicKey string.as_bytes()
    #[prost(bytes = "vec", tag = "2")]
    pub rsa_pub_key: ::prost::alloc::vec::Vec<u8>,
    /// The ephemeral Id of the wannabe pinner
    #[prost(bytes = "vec", tag = "3")]
    pub wannabe_pinner_id: ::prost::alloc::vec::Vec<u8>,
    /// The Ed25519 sig for concat bytes: wannabe_ephemeral_id + rsa_key_pub
    #[prost(bytes = "vec", tag = "4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// This uuid has no use for delegator (upstream pinner). The wannabe pinner will need this uuid to retrieve PinnerStoreItem
    #[prost(string, tag = "5")]
    pub uuid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyToBePinnerResponse {
    /// this is cid of pinner_key's pub key
    #[prost(string, tag = "1")]
    pub deployment_id: ::prost::alloc::string::String,
    /// The full pinner key encoded by rsa_pubkey from wannabe pinner.
    #[prost(bytes = "vec", tag = "2")]
    pub pinner_key_encrypted: ::prost::alloc::vec::Vec<u8>,
    /// The key1 encoded by rsa_pubkey from upstream pinner.
    #[prost(bytes = "vec", tag = "3")]
    pub key1_encrypted: ::prost::alloc::vec::Vec<u8>,
    /// use the upstream tea_id to make a broacasting chain so that we can trace back when make payment or verify
    #[prost(bytes = "vec", tag = "4")]
    pub upstream_pinner_tea_id: ::prost::alloc::vec::Vec<u8>,
    /// we use tea_id instead of ephemeral_id because this chain would be a long life time use. ephemeral key changes
    /// every time the tea node reboot.
    /// Base on today's idea, the first revenue comes from hosting as pinner will 100% go to upstream pinner. after that
    /// the revenue doens't need to share to upstream
    /// problem: what if the data is only for one time use?
    ///
    /// The Ed25519 sig for concat bytes: pinner's ephemeral key + pinner_key_enc
    #[prost(bytes = "vec", tag = "5")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// This uuid has no use for delegator (upstream pinner). The wannabe pinner will need this uuid to retrieve PinnerStoreItem
    #[prost(string, tag = "6")]
    pub uuid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WinnerExecutorCert {
    #[prost(bytes = "vec", tag = "1")]
    pub ref_num: ::prost::alloc::vec::Vec<u8>,
    /// the winner who send the application and lucky get approved by delegator
    #[prost(bytes = "vec", tag = "2")]
    pub winner_tea_id: ::prost::alloc::vec::Vec<u8>,
    /// signer is delegate, msg is tea_id(32bytes) + ref_num(32bytes)
    #[prost(bytes = "vec", tag = "3")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// Delegate decrypt eKey1, then encrypt using Executor's rsa pub key
    #[prost(bytes = "vec", tag = "4")]
    pub sec_keys_rsa_bytes: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyToExecuteTaskRequest {
    /// \[u8\] NOT a base 64 string slices. but if encoded using base64, it will become the pubsub topic.
    #[prost(bytes = "vec", tag = "1")]
    pub ref_num: ::prost::alloc::vec::Vec<u8>,
    /// Send to delegate, asking him to decrypt and send back
    #[prost(bytes = "vec", tag = "2")]
    pub ekey1: ::prost::alloc::vec::Vec<u8>,
    /// Delegate will use this RSA Pub Key to encrypt the key1 when sending to Executor. Format is tpm_protp::RsaKeyPairPemPcsk1.publicKey string.as_bytes()
    #[prost(bytes = "vec", tag = "3")]
    pub rsa_pub_key: ::prost::alloc::vec::Vec<u8>,
    /// The Tea Id of the executor who apply for this task
    #[prost(bytes = "vec", tag = "6")]
    pub executor_tea_id: ::prost::alloc::vec::Vec<u8>,
    /// The Ed25519 sig for concat bytes: executorTeaId + capcheckers\[0\] + ..+ capcheckers\[n\]
    #[prost(bytes = "vec", tag = "7")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrandExecutionRequest {
    /// The Ehpemeral Id of the delegator
    #[prost(bytes = "vec", tag = "1")]
    pub delegator_ephemeral_id: ::prost::alloc::vec::Vec<u8>,
    /// this is id of the errand
    #[prost(bytes = "vec", tag = "2")]
    pub errand_id: ::prost::alloc::vec::Vec<u8>,
    /// The Ed25519 sig for concat bytes: executorEphemeralId + expiryTime + errandId + proofOfDelegation
    #[prost(bytes = "vec", tag = "7")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// this is expiration layer1 height about this request
    #[prost(uint64, tag = "11")]
    pub expiry_time: u64,
    /// this is proof of delegation
    #[prost(bytes = "vec", tag = "12")]
    pub proof_of_delegation: ::prost::alloc::vec::Vec<u8>,
    /// this is account id of the client
    #[prost(string, tag = "13")]
    pub client_account_id: ::prost::alloc::string::String,
    /// this is payment amount for this errand
    #[prost(uint32, tag = "14")]
    pub payment: u32,
    /// tea id of the delegator
    #[prost(bytes = "vec", tag = "18")]
    pub delegator_tea_id: ::prost::alloc::vec::Vec<u8>,
    /// this is cid of the errand json
    #[prost(string, tag = "19")]
    pub errand_json_cid: ::prost::alloc::string::String,
    /// this is percentage of distribution for delegator
    #[prost(uint32, tag = "20")]
    pub delegator_percentage: u32,
    /// this is payment account about delegator
    #[prost(string, tag = "21")]
    pub delegator_payment_account: ::prost::alloc::string::String,
    #[prost(oneof = "errand_execution_request::Code", tags = "3, 4")]
    pub code: ::core::option::Option<errand_execution_request::Code>,
    #[prost(oneof = "errand_execution_request::Data", tags = "5, 6")]
    pub data: ::core::option::Option<errand_execution_request::Data>,
    #[prost(oneof = "errand_execution_request::DataSignature", tags = "8, 9, 10")]
    pub data_signature: ::core::option::Option<errand_execution_request::DataSignature>,
    #[prost(oneof = "errand_execution_request::Key3", tags = "16, 17")]
    pub key3: ::core::option::Option<errand_execution_request::Key3>,
}
/// Nested message and enum types in `ErrandExecutionRequest`.
pub mod errand_execution_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Code {
        /// this is deployment code for the errand
        #[prost(message, tag = "3")]
        DeploymentCode(super::ErrandDeploymentCode),
        /// this is adhoc code for the errand
        #[prost(message, tag = "4")]
        AdhocCode(super::ErrandAdhocCode),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// this is deployment data for the errand
        #[prost(message, tag = "5")]
        DeploymentData(super::ErrandDeploymentData),
        /// this is adhoc data for the errand
        #[prost(string, tag = "6")]
        AdhocData(::prost::alloc::string::String),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataSignature {
        /// this is signature about adhoc code
        #[prost(bytes, tag = "8")]
        Code(::prost::alloc::vec::Vec<u8>),
        /// this is signature about adhoc data
        #[prost(bytes, tag = "9")]
        Data(::prost::alloc::vec::Vec<u8>),
        /// fill this item if there is no signature
        #[prost(bool, tag = "10")]
        None(bool),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Key3 {
        /// this is bytes of key3
        #[prost(bytes, tag = "16")]
        Key3Buf(::prost::alloc::vec::Vec<u8>),
        /// set to false if there is no key3
        #[prost(bool, tag = "17")]
        HasKey3(bool),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrandExecutionFailedResponse {
    /// The Ephemeral Id of the executor
    #[prost(bytes = "vec", tag = "1")]
    pub executor_ephemeral_id: ::prost::alloc::vec::Vec<u8>,
    /// this is id of the errand
    #[prost(bytes = "vec", tag = "2")]
    pub errand_id: ::prost::alloc::vec::Vec<u8>,
    /// The Ed25519 sig for concat bytes: executorEphemeralId + errandId
    #[prost(bytes = "vec", tag = "3")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrandExecutionSucceededResponse {
    /// this is id of the errand
    #[prost(bytes = "vec", tag = "1")]
    pub errand_id: ::prost::alloc::vec::Vec<u8>,
    /// this is result of the errand execution
    #[prost(bytes = "vec", tag = "2")]
    pub result: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrandDeploymentData {
    /// this is deployment is of deployed data
    #[prost(string, tag = "1")]
    pub deployment_id: ::prost::alloc::string::String,
    /// this is payment for everytime to use this data
    #[prost(uint32, tag = "2")]
    pub pay_per_use: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrandDeploymentCode {
    /// this is deployment is of deployed code
    #[prost(string, tag = "1")]
    pub deployment_id: ::prost::alloc::string::String,
    /// this is payment for everytime to use this data
    #[prost(uint32, tag = "2")]
    pub pay_per_use: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrandAdhocCode {
    /// this is base64 encoded wasm bin without encryption
    #[prost(string, tag = "1")]
    pub code: ::prost::alloc::string::String,
    /// this is cid of cap checker wasm
    #[prost(string, tag = "2")]
    pub cap_cid: ::prost::alloc::string::String,
    /// this is description of code that including execution manifest
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrandAdhocCodeCapCheckerRequest {
    /// this is id of the errand
    #[prost(bytes = "vec", tag = "1")]
    pub errand_id: ::prost::alloc::vec::Vec<u8>,
    /// this is base64 encoded capabilities checker wasm with encryption
    #[prost(string, tag = "2")]
    pub cap_checker: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrandAdhocCodeCapCheckerResponse {
    /// The Ephemeral Id of the candidate
    #[prost(bytes = "vec", tag = "1")]
    pub candidate_ephemeral_id: ::prost::alloc::vec::Vec<u8>,
    /// this is id of the errand
    #[prost(bytes = "vec", tag = "2")]
    pub errand_id: ::prost::alloc::vec::Vec<u8>,
    /// this is cid of cap checker wasm
    #[prost(string, tag = "3")]
    pub cap_cid: ::prost::alloc::string::String,
    /// this is cap check result
    #[prost(bool, tag = "4")]
    pub result: bool,
    /// The Ed25519 sig for concat bytes: candidateEphemeralId + errandId + capCid + result
    #[prost(bytes = "vec", tag = "5")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyGenerationCandidateRequest {
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// splite the secret to `n` pieces, note we support range of u8 (0~127)
    #[prost(uint32, tag = "2")]
    pub n: u32,
    /// if have k (k < n) pieces the secret can be recovered, note we support range of u8 (0~127)
    #[prost(uint32, tag = "3")]
    pub k: u32,
    /// identify key generation type
    #[prost(string, tag = "4")]
    pub key_type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    pub delegator_ephemeral_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "6")]
    pub executor: bool,
    /// The Ed25519 sig for concat bytes: taskId + n + k + keyType + delegatorEphemeralId + executor
    #[prost(bytes = "vec", tag = "7")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskKeyGenerationApplyRequst {
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub rsa_pub_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub cap_desc: ::core::option::Option<CapabilityDescription>,
    /// decide whether to apply executor or not, if true apply executor otherwise apply initial pinner
    #[prost(bool, tag = "4")]
    pub apply_executor: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapabilityDescription {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskExecutionInitialPinnerData {
    #[prost(string, tag = "1")]
    pub peer_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub rsa_pub_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskExecutionRequest {
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub initial_pinners: ::prost::alloc::vec::Vec<TaskExecutionInitialPinnerData>,
    /// support range of u8 (0~127)
    #[prost(uint32, tag = "3")]
    pub minimum_recovery_number: u32,
    #[prost(string, tag = "4")]
    pub key_type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    pub p1_public_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskResultInitialPinnerData {
    #[prost(string, tag = "1")]
    pub peer_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub encrypted_key_slice: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskExecutionResponse {
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub initial_pinners: ::prost::alloc::vec::Vec<TaskResultInitialPinnerData>,
    #[prost(bytes = "vec", tag = "3")]
    pub p2_public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub multi_sig_account: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskPinnerKeySliceRequest {
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub encrypted_key_slice: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub multi_sig_account: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskPinnerKeySliceResponse {
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub deployment_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskSignWithKeySlicesRequst {
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub rsa_pub_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub cap_desc: ::core::option::Option<CapabilityDescription>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskSignWithKeySlicesResponse {
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub adhoc_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub p1_signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub encrypted_key_slices: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, tag = "5")]
    pub key_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskSignGetPinnerKeySliceRequest {
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// this the pub key for executor, used to encryt key slice
    #[prost(bytes = "vec", tag = "2")]
    pub rsa_pub_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub deployment_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskSignGetPinnerKeySliceResponse {
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub encrypted_key_slice: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub deployment_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskCommitSignResultRequest {
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub witness: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignCandidateRequest {
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub multi_sig_account: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "3")]
    pub n: u32,
    #[prost(uint32, tag = "4")]
    pub k: u32,
    #[prost(string, tag = "5")]
    pub task_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct P2pReplyMessage {
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub peer_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
    #[prost(enumeration = "P2pReplyType", tag = "4")]
    pub reply_type: i32,
    #[prost(message, optional, tag = "5")]
    pub reply_error: ::core::option::Option<P2pReplyError>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct P2pReplyError {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum P2pReplyType {
    Success = 0,
    Cancelled = 1,
    Rejected = 2,
    Error = 3,
}
impl P2pReplyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            P2pReplyType::Success => "Success",
            P2pReplyType::Cancelled => "Cancelled",
            P2pReplyType::Rejected => "Rejected",
            P2pReplyType::Error => "Error",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Success" => Some(Self::Success),
            "Cancelled" => Some(Self::Cancelled),
            "Rejected" => Some(Self::Rejected),
            "Error" => Some(Self::Error),
            _ => None,
        }
    }
}
