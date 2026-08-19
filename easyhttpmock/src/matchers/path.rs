use crate::mock::Request;
use caramelo::{MatchType::ToHave, Matcher, TypedMatcher};
use std::sync::Arc;

/// Creates a matcher that checks if the request path matches the given regex pattern.
///
/// # Arguments
///
/// * `value` - The regex pattern to match against.
///
/// # Returns
///
/// * `Path` - A matcher that checks if the request path matches the given regex pattern.
///
/// # Panics
///
/// * `Invalid regex pattern` - If the regex pattern is invalid.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::path;
///
/// let matcher = path(r"^/api/v1/.*$");
/// ```
pub fn path(value: &str) -> Arc<dyn TypedMatcher<Request> + Send + Sync + 'static> {
    let regex = regex::Regex::new(value);
    match regex {
        Ok(regex) => Arc::new(Path(regex)),
        Err(_) => panic!("Invalid regex pattern"),
    }
}

#[derive(Clone)]
/// A matcher that checks if the request path matches a regex pattern.
///
/// # Arguments
///
/// * `regex` - The regex pattern to match against.
///
/// # Returns
///
/// * `Path` - A matcher that checks if the request path matches the given regex pattern.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::path;
///
/// let matcher = path(r"^/api/v1/.*$");
/// ```
pub struct Path(regex::Regex);

unsafe impl Send for Path {}
unsafe impl Sync for Path {}

impl Matcher<Request> for Path {
    fn matches(&self, value: &Request) -> bool {
        self.0.is_match(
            &value
                .path()
                .to_string(),
        )
    }

    fn description(&self) -> String {
        format!("path matching {:?}", self.0)
    }
}

impl TypedMatcher<Request> for Path {
    fn matcher_type(&self) -> caramelo::MatchType {
        ToHave
    }
}
