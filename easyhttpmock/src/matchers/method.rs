use crate::mock::Request;
use caramelo::{MatchType::ToHave, Matcher, TypedMatcher};
use std::sync::Arc;

/// Trait for converting values into http::Method.
pub trait AsMethod {
    /// Converts the value into a http::Method.
    fn into_method(self) -> http::Method;
}

impl AsMethod for http::Method {
    fn into_method(self) -> http::Method {
        self
    }
}

impl AsMethod for String {
    fn into_method(self) -> http::Method {
        self.as_str()
            .into_method()
    }
}

impl AsMethod for &str {
    fn into_method(self) -> http::Method {
        match self {
            "GET" | "get" => http::Method::GET,
            "POST" | "post" => http::Method::POST,
            "QUERY" | "query" => http::Method::QUERY,
            "PUT" | "put" => http::Method::PUT,
            "DELETE" | "delete" => http::Method::DELETE,
            "PATCH" | "patch" => http::Method::PATCH,
            "HEAD" | "head" => http::Method::HEAD,
            "OPTIONS" | "options" => http::Method::OPTIONS,
            _ => panic!("Invalid method"),
        }
    }
}

/// Creates a matcher that checks if the request method matches the given method.
///
/// # Arguments
///
/// * `value` - The method to match against.
///
/// # Returns
///
/// * `Method` - A matcher that checks if the request method matches the given method.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::method;
///
/// let matcher = method("GET");
/// ```
pub fn method<M>(value: M) -> Arc<dyn TypedMatcher<Request> + Send + Sync + 'static>
where
    M: AsMethod,
{
    Arc::new(Method(value.into_method()))
}

#[derive(Clone)]
/// A matcher that checks if the request method matches a specific HTTP method.
///
/// # Arguments
///
/// * `method` - The HTTP method to match against.
///
/// # Returns
///
/// * `Method` - A matcher that checks if the request method matches the given method.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::method;
///
/// let matcher = method("GET");
/// ```
pub struct Method(http::Method);

unsafe impl Send for Method {}
unsafe impl Sync for Method {}

impl Matcher<Request> for Method {
    fn matches(&self, value: &Request) -> bool {
        self.0 == value.method()
    }

    fn description(&self) -> String {
        format!("method matching {}", self.0)
    }
}

impl TypedMatcher<Request> for Method {
    fn matcher_type(&self) -> caramelo::MatchType {
        ToHave
    }
}
