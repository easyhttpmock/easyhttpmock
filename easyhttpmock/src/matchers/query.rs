use crate::mock::Request;
use caramelo::{MatchType::ToHave, Matcher, TypedMatcher};

/// Creates a matcher that checks if the request query matches the given regex pattern.
///
/// # Arguments
///
/// * `value` - The regex pattern to match against.
///
/// # Returns
///
/// * `QueryParam` - A matcher that checks if the request query matches the given regex pattern.
///
/// # Panics
///
/// * `Invalid regex pattern` - If the regex pattern is invalid.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::query_param;
///
/// let matcher = query_param(r"^/api/v1/.*$");
/// ```
pub fn query_param(value: &str) -> QueryParam {
    let regex = regex::Regex::new(value);
    match regex {
        Ok(regex) => QueryParam(regex),
        Err(_) => panic!("Invalid regex pattern"),
    }
}

#[derive(Clone)]
/// A matcher that checks if the request query matches a regex pattern.
///
/// # Arguments
///
/// * `regex` - The regex pattern to match against.
///
/// # Returns
///
/// * `QueryParam` - A matcher that checks if the request query matches the given regex pattern.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::query_param;
///
/// let matcher = query_param(r"^.*name.*$");
/// ```
pub struct QueryParam(regex::Regex);

impl Matcher<Request> for QueryParam {
    fn matches(&self, value: &Request) -> bool {
        if let Some(query_params) = &value.query_params() {
            query_params
                .iter()
                .any(|(key, _)| self.0.is_match(key))
        } else {
            false
        }
    }

    fn description(&self) -> String {
        format!("query param matching {:?}", self.0)
    }
}

impl TypedMatcher<Request> for QueryParam {
    fn matcher_type(&self) -> caramelo::MatchType {
        ToHave
    }
}

/// Creates a matcher that checks if the request query matches the given regex pattern.
///
/// # Arguments
///
/// * `value` - The regex pattern to match against.
///
/// # Returns
///
/// * `QueryValue` - A matcher that checks if the request query matches the given regex pattern.
///
/// # Panics
///
/// * `Invalid regex pattern` - If the regex pattern is invalid.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::query_value;
///
/// let matcher = query_value(r"^/api/v1/.*$");
/// ```
pub fn query_value(value: &str) -> QueryValue {
    let regex = regex::Regex::new(value);
    match regex {
        Ok(regex) => QueryValue(regex),
        Err(_) => panic!("Invalid regex pattern"),
    }
}

#[derive(Clone)]
/// A matcher that checks if the request query matches a regex pattern.
///
/// # Arguments
///
/// * `regex` - The regex pattern to match against.
///
/// # Returns
///
/// * `QueryValue` - A matcher that checks if the request query matches the given regex pattern.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::query_value;
///
/// let matcher = query_value(r"^.*name.*$");
/// ```
pub struct QueryValue(regex::Regex);

impl Matcher<Request> for QueryValue {
    fn matches(&self, value: &Request) -> bool {
        if let Some(query_params) = &value.query_params() {
            query_params
                .iter()
                .any(|(_, value)| {
                    self.0
                        .is_match(value)
                })
        } else {
            false
        }
    }

    fn description(&self) -> String {
        format!("query value matching {:?}", self.0)
    }
}

impl TypedMatcher<Request> for QueryValue {
    fn matcher_type(&self) -> caramelo::MatchType {
        ToHave
    }
}
