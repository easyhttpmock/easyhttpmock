use crate::mock::Request;
use caramelo::{MatchType::ToHave, Matcher, TypedMatcher};
use http::HeaderName;

/// Trait for converting values into HeaderName.
pub trait AsHeaderName {
    /// Converts the value into a HeaderName.
    ///
    /// # Returns
    ///
    /// * `HeaderName` - The converted HeaderName.
    fn into_header_name(self) -> HeaderName;
}

impl AsHeaderName for HeaderName {
    fn into_header_name(self) -> HeaderName {
        self
    }
}

impl AsHeaderName for String {
    fn into_header_name(self) -> HeaderName {
        let upper = self.to_uppercase();
        match upper.parse() {
            Ok(header_name) => header_name,
            Err(_) => panic!("Invalid header name"),
        }
    }
}

impl AsHeaderName for &str {
    fn into_header_name(self) -> HeaderName {
        self.to_string()
            .into_header_name()
    }
}

/// Creates a matcher that checks if the request has the given header.
///
/// # Arguments
///
/// * `value` - The header name to match against.
///
/// # Returns
///
/// * `Header` - A matcher that checks if the request has the given header.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::header;
///
/// let matcher = header("Content-Type");
/// ```
pub fn header<H>(value: H) -> Header
where
    H: AsHeaderName,
{
    Header(value.into_header_name())
}

#[derive(Clone)]
/// A matcher that checks if the request has the given header.
///
/// # Arguments
///
/// * `name` - The header name to match against.
///
/// # Returns
///
/// * `Header` - A matcher that checks if the request has the given header.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::header;
///
/// let matcher = header("Content-Type");
/// ```
pub struct Header(http::header::HeaderName);

impl Matcher<Request> for Header {
    fn matches(&self, value: &Request) -> bool {
        value
            .headers()
            .contains_key(&self.0)
    }

    fn description(&self) -> String {
        format!("header matching {}", self.0)
    }
}

impl TypedMatcher<Request> for Header {
    fn matcher_type(&self) -> caramelo::MatchType {
        ToHave
    }
}

/// Creates a matcher that checks if the request path matches the given regex pattern.
///
/// # Arguments
///
/// * `name` - The header name to match against.
/// * `value` - The regex pattern to match against.
///
/// # Returns
///
/// * `HeaderValue` - A matcher that checks if the request path matches the given regex pattern.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::header_value;
///
/// let matcher = header_value("Content-Type", r"^application/json$");
/// ```
pub fn header_value<N>(name: N, value: &str) -> HeaderValue
where
    N: AsHeaderName,
{
    let regex = regex::Regex::new(value);
    match regex {
        Ok(regex) => HeaderValue { name: name.into_header_name(), regex },
        Err(_) => panic!("Invalid regex pattern"),
    }
}

#[derive(Clone)]
/// A matcher that checks if the request path matches a regex pattern.
///
/// # Arguments
///
/// * `name` - The header name to match against.
/// * `regex` - The regex pattern to match against.
///
/// # Returns
///
/// * `HeaderValue` - A matcher that checks if the request path matches the given regex pattern.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::header_value;
///
/// let matcher = header_value("Content-Type", r"^application/json$");
/// ```
pub struct HeaderValue {
    name: HeaderName,
    regex: regex::Regex,
}

impl Matcher<Request> for HeaderValue {
    fn matches(&self, value: &Request) -> bool {
        value
            .headers()
            .get(&self.name)
            .is_some_and(|v| {
                self.regex.is_match(
                    v.to_str()
                        .unwrap_or(""),
                )
            })
    }

    fn description(&self) -> String {
        format!("header {} with value matching {:?}", self.name, self.regex)
    }
}

impl TypedMatcher<Request> for HeaderValue {
    fn matcher_type(&self) -> caramelo::MatchType {
        ToHave
    }
}

/// Creates a matcher that checks if the request has a JWT token in the Authorization header.
///
/// # Arguments
///
/// * `token` - The JWT token to match against.
///
/// # Returns
///
/// * `Jwt` - A matcher that checks if the request has a JWT token in the Authorization header.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::jwt;
///
/// let matcher = jwt("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...");
/// ```
pub fn jwt(token: &str) -> Jwt {
    Jwt { token: token.to_string() }
}

/// A matcher that checks if the request has a JWT token in the Authorization header.
///
/// # Arguments
///
/// * `token` - The JWT token to match against.
///
/// # Returns
///
/// * `Jwt` - A matcher that checks if the request has a JWT token in the Authorization header.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::jwt;
///
/// let matcher = jwt("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...");
/// ```
#[derive(Clone)]
pub struct Jwt {
    token: String,
}

impl Matcher<Request> for Jwt {
    fn matches(&self, value: &Request) -> bool {
        value
            .headers()
            .get("Authorization")
            .is_some_and(|v| {
                v.to_str()
                    .unwrap_or("")
                    .starts_with("Bearer ")
            })
    }

    fn description(&self) -> String {
        format!("JWT token matching {}", self.token)
    }
}

impl TypedMatcher<Request> for Jwt {
    fn matcher_type(&self) -> caramelo::MatchType {
        ToHave
    }
}

/// Creates a matcher that checks if the request has a basic auth token in the Authorization header.
///
/// # Arguments
///
/// * `username` - The username to match against.
/// * `password` - The password to match against.
///
/// # Returns
///
/// * `BasicAuth` - A matcher that checks if the request has a basic auth token in the Authorization header.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::basic_auth;
///
/// let matcher = basic_auth("user", "pass");
/// ```
pub fn basic_auth(username: &str, password: &str) -> BasicAuth {
    BasicAuth { username: username.to_string(), password: password.to_string() }
}

/// A matcher that checks if the request has a basic auth token in the Authorization header.
///
/// # Arguments
///
/// * `username` - The username to match against.
/// * `password` - The password to match against.
///
/// # Returns
///
/// * `BasicAuth` - A matcher that checks if the request has a basic auth token in the Authorization header.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::basic_auth;
///
/// let matcher = basic_auth("user", "pass");
/// ```
#[derive(Clone)]
pub struct BasicAuth {
    username: String,
    password: String,
}

impl Matcher<Request> for BasicAuth {
    fn matches(&self, value: &Request) -> bool {
        value
            .headers()
            .get("Authorization")
            .is_some_and(|v| {
                v.to_str()
                    .unwrap_or("")
                    .starts_with("Basic ")
            })
    }

    fn description(&self) -> String {
        format!("Basic auth with username {} and password {}", self.username, self.password)
    }
}

impl TypedMatcher<Request> for BasicAuth {
    fn matcher_type(&self) -> caramelo::MatchType {
        ToHave
    }
}
