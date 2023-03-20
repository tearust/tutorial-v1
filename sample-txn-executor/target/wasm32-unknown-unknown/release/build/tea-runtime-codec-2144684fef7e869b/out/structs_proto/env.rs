#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub exists: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSystemTimeRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentTimestampRequest {
    #[prost(enumeration = "get_current_timestamp_request::Precision", tag = "4")]
    pub precision: i32,
    #[prost(int64, tag = "5")]
    pub trunc_base: i64,
}
/// Nested message and enum types in `GetCurrentTimestampRequest`.
pub mod get_current_timestamp_request {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Precision {
        Second = 0,
        Minute = 1,
        Hour = 2,
        Day = 3,
    }
    impl Precision {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Precision::Second => "Second",
                Precision::Minute => "Minute",
                Precision::Hour => "Hour",
                Precision::Day => "Day",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Second" => Some(Self::Second),
                "Minute" => Some(Self::Minute),
                "Hour" => Some(Self::Hour),
                "Day" => Some(Self::Day),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentTimestampResponse {
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClaimStackHashResponse {
    #[prost(string, tag = "1")]
    pub manifest: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub actors_providers_hash_list: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartTasksRequest {
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartTasksResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub system_time: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndTasksRequest {
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndTasksResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub milliseconds: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterTickRequest {
    #[prost(string, tag = "1")]
    pub subject: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub range_start: u64,
    #[prost(uint64, tag = "3")]
    pub range_end: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateProfileRequest {
    #[prost(uint64, tag = "1")]
    pub profile_seconds: u64,
    #[prost(uint64, tag = "2")]
    pub seq_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CronJobRequest {
    #[prost(message, repeated, tag = "1")]
    pub jobs: ::prost::alloc::vec::Vec<SingleCronJob>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleCronJob {
    #[prost(string, tag = "1")]
    pub subject: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub expression: ::prost::alloc::string::String,
}
