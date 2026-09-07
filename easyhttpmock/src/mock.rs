use crate::{
    matchers::{and, or},
    server::ServerAdapter,
    EasyHttpMock, HttpMockResult,
};
use bytes::Bytes;
use caramelo::{MatchType, Matcher, TypedMatcher};
use http::{request::Parts, HeaderMap, Method, StatusCode, Uri};
use std::{collections::HashMap, fmt::Debug, sync::Arc};
/// State container for mock data
pub struct MockState {
    inner: Arc<Mock>,
}

impl MockState {
    #[inline]
    /// Create a new mock state
    pub fn new(mock: Mock) -> Self {
        Self { inner: Arc::new(mock) }
    }

    /// Get a clone of the internal data Arc
    #[inline]
    pub fn inner(&self) -> Arc<Mock> {
        self.inner.clone()
    }

    /// Set the respond for this request
    pub async fn use_on<S: ServerAdapter>(
        self,
        server: &mut EasyHttpMock<S>,
    ) -> HttpMockResult<()> {
        server
            .register_mock(self)
            .await?;

        Ok(())
    }
}

/// Mock struct
pub struct Mock {
    request: RequestMock,
}

impl Mock {
    #[inline]
    /// Create a new mock
    pub fn of(request: RequestMock) -> MockState {
        MockState::new(Self { request })
    }

    #[inline]
    /// Get the request mock
    pub fn request(&self) -> &RequestMock {
        &self.request
    }
}

#[inline]
/// Add a matcher to this request
pub fn given(matcher: impl TypedMatcher<Request> + Send + Sync + 'static) -> RequestMock {
    RequestMock { matcher: Arc::from(matcher), respond: None }
}

/// Represents a mock request
pub struct RequestMock {
    matcher: Arc<dyn TypedMatcher<Request> + Send + Sync + 'static>,
    respond: Option<Respond>,
}

impl RequestMock {
    #[inline]
    /// Get the matcher for this request
    pub fn matcher(&self) -> &Arc<dyn TypedMatcher<Request> + Send + Sync + 'static> {
        &self.matcher
    }

    #[inline]
    /// Get the respond for this request
    pub fn respond(&self) -> Option<&Respond> {
        self.respond
            .as_ref()
    }

    #[inline]
    /// Set the response for this request
    pub fn will_return(mut self, respond: Respond) -> Self {
        self.respond = Some(respond);
        self
    }
}

impl Matcher<Request> for Arc<dyn TypedMatcher<Request> + Send + Sync + 'static> {
    fn matches(&self, request: &Request) -> bool {
        self.as_ref()
            .matches(request)
    }

    fn description(&self) -> String {
        self.as_ref()
            .description()
    }
}

impl TypedMatcher<Request> for Arc<dyn TypedMatcher<Request> + Send + Sync + 'static> {
    fn matcher_type(&self) -> MatchType {
        self.as_ref()
            .matcher_type()
    }
}
/// Extension trait for adding AND/OR combinators to matchers
pub trait AsyncMatcherExt<T>: TypedMatcher<T> + Sized + 'static
where
    T: Send + Sync + 'static,
{
    /// Combines this matcher with another using AND logic
    fn and<M>(self, matcher: M) -> Arc<dyn TypedMatcher<T> + Send + Sync + 'static>
    where
        Self: Send + Sync,
        M: TypedMatcher<T> + Send + Sync + 'static,
    {
        and(vec![Arc::new(self), Arc::new(matcher)])
    }

    /// Combines this matcher with another using OR logic
    fn or<M>(self, matcher: M) -> Arc<dyn TypedMatcher<T> + Send + Sync + 'static>
    where
        Self: Send + Sync,
        M: TypedMatcher<T> + Send + Sync + 'static,
    {
        or(vec![Arc::new(self), Arc::new(matcher)])
    }
}

/// Implementation of AsyncMatcherExt for any type that implements TypedMatcher
impl<M, T> AsyncMatcherExt<T> for M
where
    M: TypedMatcher<T> + Send + Sync + 'static,
    T: Send + Sync + 'static,
{
}

/// Extension trait for StatusCode to create responses
pub trait StatusCodeExt {
    /// Create a response builder with this status code
    fn respond(self) -> RespondBuilder;
}

impl StatusCodeExt for StatusCode {
    /// Create a response builder with this status code
    fn respond(self) -> RespondBuilder {
        RespondBuilder { status_code: self, headers: HashMap::new() }
    }
}

/// A builder for creating http requests
#[derive(Debug)]
pub struct RequestBuilder {
    uri: Uri,
    query_params: Option<HashMap<String, String>>,
    method: http::Method,
    version: http::Version,
    headers: http::HeaderMap,
}

impl RequestBuilder {
    /// Sets the request path
    pub fn uri(self, uri: Uri) -> Self {
        Self { uri, ..self }
    }

    /// Sets the request query parameters
    pub fn query_params(self, query_params: HashMap<String, String>) -> Self {
        Self { query_params: Some(query_params), ..self }
    }

    /// Sets the request method
    pub fn method(self, method: http::Method) -> Self {
        Self { method, ..self }
    }

    /// Sets the request version
    pub fn version(self, version: http::Version) -> Self {
        Self { version, ..self }
    }

    /// Sets the request headers
    pub fn header<K>(self, name: K, value: &str) -> Self
    where
        K: http::header::IntoHeaderName,
    {
        let mut headers = self.headers;
        headers.insert(
            name,
            value
                .parse()
                .unwrap(),
        );
        Self { headers, ..self }
    }

    /// Creates an empty request
    pub fn empty(self) -> Result<Request, http::Error> {
        Ok(Request {
            method: self.method,
            version: self.version,
            query_params: self.query_params,
            uri: self.uri,
            headers: self.headers,
            body: None,
        })
    }

    /// Builds the request
    pub fn body(self, bytes: Bytes) -> Result<Request, http::Error> {
        Ok(Request {
            method: self.method,
            version: self.version,
            query_params: self.query_params,
            uri: self.uri,
            headers: self.headers,
            body: Some(bytes),
        })
    }
}

/// Represents a mock HTTP request
#[derive(Clone, Debug, PartialEq)]
pub struct Request {
    method: Method,
    uri: Uri,
    version: http::Version,
    headers: HeaderMap,
    query_params: Option<HashMap<String, String>>,
    body: Option<Bytes>,
}

impl Request {
    #[inline]
    /// Create a new request builder
    pub fn from_parts(parts: Parts) -> Request {
        let query_params = parts
            .uri
            .query()
            .map(|q| {
                q.split('&')
                    .filter_map(|pair| {
                        pair.split_once('=')
                            .map(|(k, v)| (k.to_string(), v.to_string()))
                    })
                    .collect()
            });

        Request {
            method: parts.method,
            uri: parts.uri,
            version: parts.version,
            headers: parts.headers,
            query_params,
            body: None,
        }
    }

    fn builder(method: http::Method, uri: Uri) -> RequestBuilder {
        RequestBuilder {
            method,
            version: http::Version::HTTP_11,
            uri,
            headers: http::HeaderMap::new(),
            query_params: None,
        }
    }

    /// Creates a new GET request builder
    pub fn get(uri: Uri) -> RequestBuilder {
        Self::builder(http::Method::GET, uri)
    }

    /// Creates a new POST request builder
    pub fn post(uri: Uri) -> RequestBuilder {
        Self::builder(http::Method::POST, uri)
    }

    /// Creates a new PUT request builder
    pub fn put(uri: Uri) -> RequestBuilder {
        Self::builder(http::Method::PUT, uri)
    }

    /// Creates a new DELETE request builder
    pub fn delete(uri: Uri) -> RequestBuilder {
        Self::builder(http::Method::DELETE, uri)
    }

    /// Creates a new PATCH request builder
    pub fn patch(uri: Uri) -> RequestBuilder {
        Self::builder(http::Method::PATCH, uri)
    }

    /// Creates a new HEAD request builder
    pub fn head(uri: Uri) -> RequestBuilder {
        Self::builder(http::Method::HEAD, uri)
    }

    /// Creates a new OPTIONS request builder
    pub fn options(uri: Uri) -> RequestBuilder {
        Self::builder(http::Method::OPTIONS, uri)
    }

    /// Creates a new TRACE request builder
    pub fn trace(uri: Uri) -> RequestBuilder {
        Self::builder(http::Method::TRACE, uri)
    }

    /// Creates a new CONNECT request builder
    pub fn connect(uri: Uri) -> RequestBuilder {
        Self::builder(http::Method::CONNECT, uri)
    }

    #[inline]
    /// Get the path
    pub fn path(&self) -> &Uri {
        &self.uri
    }

    #[inline]
    /// Get the method
    pub fn method(&self) -> &Method {
        &self.method
    }

    #[inline]
    /// Get the headers
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    #[inline]
    /// Get the version
    pub fn version(&self) -> &http::Version {
        &self.version
    }

    #[inline]
    /// Get the query params
    pub fn query_params(&self) -> &Option<HashMap<String, String>> {
        &self.query_params
    }

    #[inline]
    /// Get the body
    pub fn body(&self) -> &Option<Bytes> {
        &self.body
    }
}
/// Builder for what represents a response for a request
pub struct RespondBuilder {
    status_code: StatusCode,
    headers: HashMap<String, String>,
}

impl RespondBuilder {
    #[inline]
    /// Set the status code for this response
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status_code = status;
        self
    }

    #[inline]
    /// Set a header for this response
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers
            .insert(key.to_string(), value.to_string());
        self
    }

    #[inline]
    /// Set multiple headers for this response
    pub fn with_headers(mut self, entries: &[(&str, &str)]) -> Self {
        for (key, value) in entries {
            self.headers
                .insert(key.to_string(), value.to_string());
        }
        self
    }

    #[inline]
    /// Create an empty response
    pub fn empty(self) -> Respond {
        self.no_body()
    }

    #[inline]
    /// Create a response with no body
    pub fn no_body(self) -> Respond {
        Respond { status_code: self.status_code, headers: self.headers, body: Bytes::new() }
    }

    #[inline]
    /// Create a response with a body
    pub fn with_body(self, body: &[u8]) -> Respond {
        Respond {
            status_code: self.status_code,
            headers: self.headers,
            body: Bytes::from(body.to_vec()),
        }
    }
}

/// Represents how to respond for a request
#[derive(Clone, Debug, PartialEq)]
pub struct Respond {
    status_code: StatusCode,
    headers: HashMap<String, String>,
    body: Bytes,
}

impl Respond {
    /// Initialize respond builder
    #[inline]
    /// Initialize respond builder
    pub fn builder() -> RespondBuilder {
        RespondBuilder { status_code: StatusCode::OK, headers: HashMap::new() }
    }

    #[inline]
    /// Get the status code
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    #[inline]
    /// Get the headers
    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    #[inline]
    /// Get the body
    pub fn body(&self) -> Bytes {
        self.body.clone()
    }
}
