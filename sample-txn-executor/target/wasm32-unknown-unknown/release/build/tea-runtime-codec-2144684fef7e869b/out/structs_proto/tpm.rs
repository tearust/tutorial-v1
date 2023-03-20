#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifySignatureRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifySignatureResponse {
    #[prost(bool, tag = "1")]
    pub result: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ed25519SignRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ed25519SignResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sr25519SignRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sr25519SignResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RsaDecryptRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub private_key_pkcs1: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RsaDecryptResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub result: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RsaEncryptRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub public_key_pkcs1: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RsaEncryptResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub result: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPcrRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPcrResponse {
    #[prost(message, optional, tag = "1")]
    pub pcrs: ::core::option::Option<Pcrs>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RsaKeyPairPemPcsk1 {
    #[prost(string, tag = "1")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub private_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedPcrsBytes {
    #[prost(message, optional, tag = "1")]
    pub pcrs: ::core::option::Option<Pcrs>,
    #[prost(bytes = "vec", tag = "2")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TeaNodeProfileV1 {
    #[prost(message, optional, tag = "1")]
    pub pcr_values: ::core::option::Option<SignedPcrsBytes>,
    /// required string actorsProvidersHashList = 4;
    #[prost(string, tag = "3")]
    pub manifest: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TeaNodeProfileV2 {
    #[prost(message, optional, tag = "1")]
    pub pcr_values: ::core::option::Option<SignedPcrsBytes>,
    #[prost(string, tag = "3")]
    pub manifest: ::prost::alloc::string::String,
    /// required string actorsProvidersHashList = 4;
    #[prost(string, tag = "4")]
    pub mode: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pcrs {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub pcrs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebuildOtherPeerPcrBytesRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub ephemeral_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub manifest: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseError {
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEd25519KeyPairRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEd25519KeyPairResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub keypair: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEphemeralIdKeyPairSigRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEphemeralIdKeyPairSigResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub pubkey: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub prikey: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTeaIdRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTeaIdResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub tea_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSignedPcrRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSignedPcrResponse {
    #[prost(message, optional, tag = "1")]
    pub pcrs: ::core::option::Option<Pcrs>,
    #[prost(bytes = "vec", tag = "2")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendPcrRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub new_pcr: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendPcrResponse {
    #[prost(bool, tag = "1")]
    pub result: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TpmMessage {
    #[prost(
        oneof = "tpm_message::Msg",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21"
    )]
    pub msg: ::core::option::Option<tpm_message::Msg>,
}
/// Nested message and enum types in `TpmMessage`.
pub mod tpm_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "1")]
        ResponseError(super::ResponseError),
        #[prost(message, tag = "2")]
        RasEncryptRequest(super::RsaEncryptRequest),
        #[prost(message, tag = "3")]
        RsaEncryptResponse(super::RsaEncryptResponse),
        #[prost(message, tag = "4")]
        ExtendPcrRequest(super::ExtendPcrRequest),
        #[prost(message, tag = "5")]
        ExtendPcrResponse(super::ExtendPcrResponse),
        #[prost(message, tag = "6")]
        VerifySignatureRequest(super::VerifySignatureRequest),
        #[prost(message, tag = "7")]
        VerifySignatureResponse(super::VerifySignatureResponse),
        #[prost(message, tag = "8")]
        Ed25519SignRequest(super::Ed25519SignRequest),
        #[prost(message, tag = "9")]
        Ed25519SignResponse(super::Ed25519SignResponse),
        #[prost(message, tag = "10")]
        Sr25519SignRequest(super::Sr25519SignRequest),
        #[prost(message, tag = "11")]
        Sr25519SignResponse(super::Sr25519SignResponse),
        #[prost(message, tag = "12")]
        RsaDecryptRequest(super::RsaDecryptRequest),
        #[prost(message, tag = "13")]
        RsaDecryptResponse(super::RsaDecryptResponse),
        #[prost(message, tag = "14")]
        GetEd25519KeyPairRequest(super::GetEd25519KeyPairRequest),
        #[prost(message, tag = "15")]
        GetEd25519KeyPairResponse(super::GetEd25519KeyPairResponse),
        #[prost(message, tag = "16")]
        GetEphemeralIdKeyPairSigRequest(super::GetEphemeralIdKeyPairSigRequest),
        #[prost(message, tag = "17")]
        GetEphemeralIdKeyPairSigResponse(super::GetEphemeralIdKeyPairSigResponse),
        #[prost(message, tag = "18")]
        GetTeaIdRequest(super::GetTeaIdRequest),
        #[prost(message, tag = "19")]
        GetTeaIdResponse(super::GetTeaIdResponse),
        #[prost(message, tag = "20")]
        GetSignedPcrRequest(super::GetSignedPcrRequest),
        #[prost(message, tag = "21")]
        GetSignedPcrResponse(super::GetSignedPcrResponse),
    }
}
