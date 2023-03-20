#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportEvidenceRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub commit_tea_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub phishing_tea_id: ::prost::alloc::vec::Vec<u8>,
}
