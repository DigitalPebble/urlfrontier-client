/// *
/// Message returned by the GetStats method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stats {
    /// number of active URLs in queues
    #[prost(uint64, tag = "1")]
    pub size: u64,
    /// number of URLs currently in flight
    #[prost(uint32, tag = "2")]
    pub in_process: u32,
    /// custom counts
    #[prost(map = "string, uint64", tag = "3")]
    pub counts: ::std::collections::HashMap<::prost::alloc::string::String, u64>,
    /// number of active queues in the frontier
    #[prost(uint64, tag = "4")]
    pub number_of_queues: u64,
    /// crawl ID
    #[prost(string, tag = "5")]
    pub crawl_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pagination {
    /// position of the first result in the list; defaults to 0
    #[prost(uint32, tag = "1")]
    pub start: u32,
    /// max number of values; defaults to 100
    #[prost(uint32, tag = "2")]
    pub size: u32,
    /// include inactive queues; defaults to false
    #[prost(bool, tag = "3")]
    pub include_inactive: bool,
    /// crawl ID
    #[prost(string, tag = "4")]
    pub crawl_id: ::prost::alloc::string::String,
    /// only for the current local instance
    #[prost(bool, tag = "5")]
    pub local: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCrawlMessage {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub local: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Local {
    #[prost(bool, tag = "1")]
    pub local: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Active {
    #[prost(bool, tag = "1")]
    pub state: bool,
    #[prost(bool, tag = "2")]
    pub local: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Boolean {
    #[prost(bool, tag = "1")]
    pub state: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Long {
    #[prost(uint64, tag = "1")]
    pub value: u64,
}
/// * Returned by ListQueues *
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueList {
    #[prost(string, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// total number of queues
    #[prost(uint64, tag = "2")]
    pub total: u64,
    /// position of the first result in the list
    #[prost(uint32, tag = "3")]
    pub start: u32,
    /// number of values returned
    #[prost(uint32, tag = "4")]
    pub size: u32,
    /// crawl ID - empty string for default
    #[prost(string, tag = "5")]
    pub crawl_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringList {
    #[prost(string, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueWithinCrawlParams {
    /// * ID for the queue *
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// crawl ID - empty string for default
    #[prost(string, tag = "2")]
    pub crawl_id: ::prost::alloc::string::String,
    /// only for this instance
    #[prost(bool, tag = "3")]
    pub local: bool,
}
/// * Parameter message for SetDelay *
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueDelayParams {
    /// * ID for the queue - an empty value sets the default for all the queues *
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    ///   delay in seconds before a queue can provide new URLs
    #[prost(uint32, tag = "2")]
    pub delay_requestable: u32,
    /// crawl ID - empty string for default
    #[prost(string, tag = "3")]
    pub crawl_id: ::prost::alloc::string::String,
    /// only for this instance
    #[prost(bool, tag = "4")]
    pub local: bool,
}
/// * Parameter message for BlockQueueUntil *
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockQueueParams {
    /// * ID for the queue *
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// * Expressed in seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. The default value of 0 will unblock the queue.
    #[prost(uint64, tag = "2")]
    pub time: u64,
    /// crawl ID
    #[prost(string, tag = "3")]
    pub crawl_id: ::prost::alloc::string::String,
    /// only for this instance
    #[prost(bool, tag = "4")]
    pub local: bool,
}
/// * Parameter message for GetURLs *
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParams {
    /// maximum number of URLs per queue, the default value of 0 means no limit
    #[prost(uint32, tag = "1")]
    pub max_urls_per_queue: u32,
    /// maximum number of queues to get URLs from, the default value of 0 means no limit
    #[prost(uint32, tag = "2")]
    pub max_queues: u32,
    /// queue id if restricting to a specific queue
    #[prost(string, tag = "3")]
    pub key: ::prost::alloc::string::String,
    ///   delay in seconds before a URL can be unlocked and sent again for fetching
    #[prost(uint32, tag = "4")]
    pub delay_requestable: u32,
    #[prost(oneof = "get_params::Item", tags = "5, 6")]
    pub item: ::core::option::Option<get_params::Item>,
}
/// Nested message and enum types in `GetParams`.
pub mod get_params {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        #[prost(message, tag = "5")]
        AnyCrawlId(super::AnyCrawlId),
        #[prost(string, tag = "6")]
        CrawlId(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnyCrawlId {}
/// * Wrapper for a KnownURLItem or DiscoveredURLItem *
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UrlItem {
    /// * Identifier specified by the client, if missing, the URL is returned *
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    #[prost(oneof = "url_item::Item", tags = "1, 2")]
    pub item: ::core::option::Option<url_item::Item>,
}
/// Nested message and enum types in `URLItem`.
pub mod url_item {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        #[prost(message, tag = "1")]
        Discovered(super::DiscoveredUrlItem),
        #[prost(message, tag = "2")]
        Known(super::KnownUrlItem),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AckMessage {
    /// * ID which had been specified by the client *
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration = "ack_message::Status", tag = "2")]
    pub status: i32,
}
/// Nested message and enum types in `AckMessage`.
pub mod ack_message {
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
    pub enum Status {
        Ok = 0,
        Skipped = 1,
        Fail = 2,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Ok => "OK",
                Status::Skipped => "SKIPPED",
                Status::Fail => "FAIL",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UrlInfo {
    /// * URL *
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// * The key is used to put the URLs into queues, the value can be anything set by the client but would typically be the hostname,
    /// domain name or IP or the URL. If not set, the service will use a sensible default like hostname.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// *
    /// Arbitrary key / values stored alongside the URL. Can be anything needed by the crawler like http status, source URL etc...
    #[prost(map = "string, message", tag = "3")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        StringList,
    >,
    /// * crawl ID *
    #[prost(string, tag = "4")]
    pub crawl_id: ::prost::alloc::string::String,
}
/// *
/// URL which was already known in the frontier, was returned by GetURLs() and processed by the crawler. Used for updating the information
/// about it in the frontier. If the date is not set, the URL will be considered done and won't be resubmitted for fetching, otherwise
/// it will be elligible for fetching after the delay has elapsed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KnownUrlItem {
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<UrlInfo>,
    /// * Expressed in seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. Optional, the default value of 0 indicates
    /// that a URL should not be refetched.
    #[prost(uint64, tag = "2")]
    pub refetchable_from_date: u64,
}
/// *
/// URL discovered during the crawl, might already be known in the URL Frontier or not.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveredUrlItem {
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<UrlInfo>,
}
/// *
/// Configuration of the log level for a particular package, e.g.
/// crawlercommons.urlfrontier.service.rocksdb DEBUG
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogLevelParams {
    #[prost(string, tag = "1")]
    pub package: ::prost::alloc::string::String,
    #[prost(enumeration = "log_level_params::Level", tag = "2")]
    pub level: i32,
    /// only for this instance
    #[prost(bool, tag = "3")]
    pub local: bool,
}
/// Nested message and enum types in `LogLevelParams`.
pub mod log_level_params {
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
    pub enum Level {
        Trace = 0,
        Debug = 1,
        Info = 2,
        Warn = 3,
        Error = 4,
    }
    impl Level {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Level::Trace => "TRACE",
                Level::Debug => "DEBUG",
                Level::Info => "INFO",
                Level::Warn => "WARN",
                Level::Error => "ERROR",
            }
        }
    }
}
/// Generated client implementations.
pub mod url_frontier_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct UrlFrontierClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UrlFrontierClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> UrlFrontierClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UrlFrontierClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            UrlFrontierClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// * Return the list of nodes forming the cluster the current node belongs to *
        pub async fn list_nodes(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> Result<tonic::Response<super::StringList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/urlfrontier.URLFrontier/ListNodes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// * Return the list of crawls handled by the frontier(s) *
        pub async fn list_crawls(
            &mut self,
            request: impl tonic::IntoRequest<super::Local>,
        ) -> Result<tonic::Response<super::StringList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/urlfrontier.URLFrontier/ListCrawls",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// * Delete an entire crawl, returns the number of URLs removed this way *
        pub async fn delete_crawl(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCrawlMessage>,
        ) -> Result<tonic::Response<super::Long>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/urlfrontier.URLFrontier/DeleteCrawl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// * Return a list of queues for a specific crawl. Can chose whether to include inactive queues (a queue is active if it has URLs due for fetching);
        /// by default the service will return up to 100 results from offset 0 and exclude inactive queues.*
        pub async fn list_queues(
            &mut self,
            request: impl tonic::IntoRequest<super::Pagination>,
        ) -> Result<tonic::Response<super::QueueList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/urlfrontier.URLFrontier/ListQueues",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// * Stream URLs due for fetching from M queues with up to N items per queue *
        pub async fn get_ur_ls(
            &mut self,
            request: impl tonic::IntoRequest<super::GetParams>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::UrlInfo>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/urlfrontier.URLFrontier/GetURLs",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// * Push URL items to the server; they get created (if they don't already exist) in case of DiscoveredURLItems or updated if KnownURLItems *
        pub async fn put_ur_ls(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::UrlItem>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::AckMessage>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/urlfrontier.URLFrontier/PutURLs",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        /// * Return stats for a specific queue or an entire crawl. Does not aggregate the stats across different crawlids. *
        pub async fn get_stats(
            &mut self,
            request: impl tonic::IntoRequest<super::QueueWithinCrawlParams>,
        ) -> Result<tonic::Response<super::Stats>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/urlfrontier.URLFrontier/GetStats",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// * Delete the queue based on the key in parameter, returns the number of URLs removed this way *
        pub async fn delete_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::QueueWithinCrawlParams>,
        ) -> Result<tonic::Response<super::Long>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/urlfrontier.URLFrontier/DeleteQueue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// * Block a queue from sending URLs; the argument is the number of seconds of UTC time since Unix epoch
        /// 1970-01-01T00:00:00Z. The default value of 0 will unblock the queue. The block will get removed once the time
        /// indicated in argument is reached. This is useful for cases where a server returns a Retry-After for instance.
        pub async fn block_queue_until(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockQueueParams>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/urlfrontier.URLFrontier/BlockQueueUntil",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// * De/activate the crawl. GetURLs will not return anything until SetActive is set to true. PutURLs will still take incoming data. *
        pub async fn set_active(
            &mut self,
            request: impl tonic::IntoRequest<super::Active>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/urlfrontier.URLFrontier/SetActive",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// * Returns true if the crawl is active, false if it has been deactivated with SetActive(Boolean) *
        pub async fn get_active(
            &mut self,
            request: impl tonic::IntoRequest<super::Local>,
        ) -> Result<tonic::Response<super::Boolean>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/urlfrontier.URLFrontier/GetActive",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// * Set a delay from a given queue.
        /// No URLs will be obtained via GetURLs for this queue until the number of seconds specified has
        /// elapsed since the last time URLs were retrieved.
        /// Usually informed by the delay setting of robots.txt.
        pub async fn set_delay(
            &mut self,
            request: impl tonic::IntoRequest<super::QueueDelayParams>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/urlfrontier.URLFrontier/SetDelay",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// * Overrides the log level for a given package *
        pub async fn set_log_level(
            &mut self,
            request: impl tonic::IntoRequest<super::LogLevelParams>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/urlfrontier.URLFrontier/SetLogLevel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod url_frontier_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with UrlFrontierServer.
    #[async_trait]
    pub trait UrlFrontier: Send + Sync + 'static {
        /// * Return the list of nodes forming the cluster the current node belongs to *
        async fn list_nodes(
            &self,
            request: tonic::Request<super::Empty>,
        ) -> Result<tonic::Response<super::StringList>, tonic::Status>;
        /// * Return the list of crawls handled by the frontier(s) *
        async fn list_crawls(
            &self,
            request: tonic::Request<super::Local>,
        ) -> Result<tonic::Response<super::StringList>, tonic::Status>;
        /// * Delete an entire crawl, returns the number of URLs removed this way *
        async fn delete_crawl(
            &self,
            request: tonic::Request<super::DeleteCrawlMessage>,
        ) -> Result<tonic::Response<super::Long>, tonic::Status>;
        /// * Return a list of queues for a specific crawl. Can chose whether to include inactive queues (a queue is active if it has URLs due for fetching);
        /// by default the service will return up to 100 results from offset 0 and exclude inactive queues.*
        async fn list_queues(
            &self,
            request: tonic::Request<super::Pagination>,
        ) -> Result<tonic::Response<super::QueueList>, tonic::Status>;
        /// Server streaming response type for the GetURLs method.
        type GetURLsStream: futures_core::Stream<
                Item = Result<super::UrlInfo, tonic::Status>,
            >
            + Send
            + 'static;
        /// * Stream URLs due for fetching from M queues with up to N items per queue *
        async fn get_ur_ls(
            &self,
            request: tonic::Request<super::GetParams>,
        ) -> Result<tonic::Response<Self::GetURLsStream>, tonic::Status>;
        /// Server streaming response type for the PutURLs method.
        type PutURLsStream: futures_core::Stream<
                Item = Result<super::AckMessage, tonic::Status>,
            >
            + Send
            + 'static;
        /// * Push URL items to the server; they get created (if they don't already exist) in case of DiscoveredURLItems or updated if KnownURLItems *
        async fn put_ur_ls(
            &self,
            request: tonic::Request<tonic::Streaming<super::UrlItem>>,
        ) -> Result<tonic::Response<Self::PutURLsStream>, tonic::Status>;
        /// * Return stats for a specific queue or an entire crawl. Does not aggregate the stats across different crawlids. *
        async fn get_stats(
            &self,
            request: tonic::Request<super::QueueWithinCrawlParams>,
        ) -> Result<tonic::Response<super::Stats>, tonic::Status>;
        /// * Delete the queue based on the key in parameter, returns the number of URLs removed this way *
        async fn delete_queue(
            &self,
            request: tonic::Request<super::QueueWithinCrawlParams>,
        ) -> Result<tonic::Response<super::Long>, tonic::Status>;
        /// * Block a queue from sending URLs; the argument is the number of seconds of UTC time since Unix epoch
        /// 1970-01-01T00:00:00Z. The default value of 0 will unblock the queue. The block will get removed once the time
        /// indicated in argument is reached. This is useful for cases where a server returns a Retry-After for instance.
        async fn block_queue_until(
            &self,
            request: tonic::Request<super::BlockQueueParams>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status>;
        /// * De/activate the crawl. GetURLs will not return anything until SetActive is set to true. PutURLs will still take incoming data. *
        async fn set_active(
            &self,
            request: tonic::Request<super::Active>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status>;
        /// * Returns true if the crawl is active, false if it has been deactivated with SetActive(Boolean) *
        async fn get_active(
            &self,
            request: tonic::Request<super::Local>,
        ) -> Result<tonic::Response<super::Boolean>, tonic::Status>;
        /// * Set a delay from a given queue.
        /// No URLs will be obtained via GetURLs for this queue until the number of seconds specified has
        /// elapsed since the last time URLs were retrieved.
        /// Usually informed by the delay setting of robots.txt.
        async fn set_delay(
            &self,
            request: tonic::Request<super::QueueDelayParams>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status>;
        /// * Overrides the log level for a given package *
        async fn set_log_level(
            &self,
            request: tonic::Request<super::LogLevelParams>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct UrlFrontierServer<T: UrlFrontier> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: UrlFrontier> UrlFrontierServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UrlFrontierServer<T>
    where
        T: UrlFrontier,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/urlfrontier.URLFrontier/ListNodes" => {
                    #[allow(non_camel_case_types)]
                    struct ListNodesSvc<T: UrlFrontier>(pub Arc<T>);
                    impl<T: UrlFrontier> tonic::server::UnaryService<super::Empty>
                    for ListNodesSvc<T> {
                        type Response = super::StringList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Empty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_nodes(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListNodesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/urlfrontier.URLFrontier/ListCrawls" => {
                    #[allow(non_camel_case_types)]
                    struct ListCrawlsSvc<T: UrlFrontier>(pub Arc<T>);
                    impl<T: UrlFrontier> tonic::server::UnaryService<super::Local>
                    for ListCrawlsSvc<T> {
                        type Response = super::StringList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Local>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_crawls(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListCrawlsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/urlfrontier.URLFrontier/DeleteCrawl" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCrawlSvc<T: UrlFrontier>(pub Arc<T>);
                    impl<
                        T: UrlFrontier,
                    > tonic::server::UnaryService<super::DeleteCrawlMessage>
                    for DeleteCrawlSvc<T> {
                        type Response = super::Long;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteCrawlMessage>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_crawl(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteCrawlSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/urlfrontier.URLFrontier/ListQueues" => {
                    #[allow(non_camel_case_types)]
                    struct ListQueuesSvc<T: UrlFrontier>(pub Arc<T>);
                    impl<T: UrlFrontier> tonic::server::UnaryService<super::Pagination>
                    for ListQueuesSvc<T> {
                        type Response = super::QueueList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Pagination>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_queues(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListQueuesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/urlfrontier.URLFrontier/GetURLs" => {
                    #[allow(non_camel_case_types)]
                    struct GetURLsSvc<T: UrlFrontier>(pub Arc<T>);
                    impl<
                        T: UrlFrontier,
                    > tonic::server::ServerStreamingService<super::GetParams>
                    for GetURLsSvc<T> {
                        type Response = super::UrlInfo;
                        type ResponseStream = T::GetURLsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_ur_ls(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetURLsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/urlfrontier.URLFrontier/PutURLs" => {
                    #[allow(non_camel_case_types)]
                    struct PutURLsSvc<T: UrlFrontier>(pub Arc<T>);
                    impl<T: UrlFrontier> tonic::server::StreamingService<super::UrlItem>
                    for PutURLsSvc<T> {
                        type Response = super::AckMessage;
                        type ResponseStream = T::PutURLsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::UrlItem>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).put_ur_ls(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PutURLsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/urlfrontier.URLFrontier/GetStats" => {
                    #[allow(non_camel_case_types)]
                    struct GetStatsSvc<T: UrlFrontier>(pub Arc<T>);
                    impl<
                        T: UrlFrontier,
                    > tonic::server::UnaryService<super::QueueWithinCrawlParams>
                    for GetStatsSvc<T> {
                        type Response = super::Stats;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueueWithinCrawlParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_stats(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetStatsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/urlfrontier.URLFrontier/DeleteQueue" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteQueueSvc<T: UrlFrontier>(pub Arc<T>);
                    impl<
                        T: UrlFrontier,
                    > tonic::server::UnaryService<super::QueueWithinCrawlParams>
                    for DeleteQueueSvc<T> {
                        type Response = super::Long;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueueWithinCrawlParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_queue(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteQueueSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/urlfrontier.URLFrontier/BlockQueueUntil" => {
                    #[allow(non_camel_case_types)]
                    struct BlockQueueUntilSvc<T: UrlFrontier>(pub Arc<T>);
                    impl<
                        T: UrlFrontier,
                    > tonic::server::UnaryService<super::BlockQueueParams>
                    for BlockQueueUntilSvc<T> {
                        type Response = super::Empty;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockQueueParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).block_queue_until(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BlockQueueUntilSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/urlfrontier.URLFrontier/SetActive" => {
                    #[allow(non_camel_case_types)]
                    struct SetActiveSvc<T: UrlFrontier>(pub Arc<T>);
                    impl<T: UrlFrontier> tonic::server::UnaryService<super::Active>
                    for SetActiveSvc<T> {
                        type Response = super::Empty;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Active>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_active(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetActiveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/urlfrontier.URLFrontier/GetActive" => {
                    #[allow(non_camel_case_types)]
                    struct GetActiveSvc<T: UrlFrontier>(pub Arc<T>);
                    impl<T: UrlFrontier> tonic::server::UnaryService<super::Local>
                    for GetActiveSvc<T> {
                        type Response = super::Boolean;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Local>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_active(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetActiveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/urlfrontier.URLFrontier/SetDelay" => {
                    #[allow(non_camel_case_types)]
                    struct SetDelaySvc<T: UrlFrontier>(pub Arc<T>);
                    impl<
                        T: UrlFrontier,
                    > tonic::server::UnaryService<super::QueueDelayParams>
                    for SetDelaySvc<T> {
                        type Response = super::Empty;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueueDelayParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_delay(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetDelaySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/urlfrontier.URLFrontier/SetLogLevel" => {
                    #[allow(non_camel_case_types)]
                    struct SetLogLevelSvc<T: UrlFrontier>(pub Arc<T>);
                    impl<
                        T: UrlFrontier,
                    > tonic::server::UnaryService<super::LogLevelParams>
                    for SetLogLevelSvc<T> {
                        type Response = super::Empty;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogLevelParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_log_level(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetLogLevelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: UrlFrontier> Clone for UrlFrontierServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: UrlFrontier> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UrlFrontier> tonic::server::NamedService for UrlFrontierServer<T> {
        const NAME: &'static str = "urlfrontier.URLFrontier";
    }
}
