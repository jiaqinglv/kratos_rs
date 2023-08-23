#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reply {
    /// 状态
    #[prost(bool, tag = "1")]
    pub status: bool,
    /// 状态码
    #[prost(int64, tag = "2")]
    pub code: i64,
    /// 消息
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
