use caramelo::{
    MatchType::{self, To},
    Matcher, TypedMatcher,
};
use std::sync::Arc;

/// Creates a matcher that matches values that satisfy all given matchers
///
/// # Examples
///
/// ```
/// use caramelo::{and, expect};
/// use caramelo::matchers::{contains};
///
/// expect("hello").to_match(and!(contains("ell"), contains("llo")));
/// ```
pub fn and<T: Send + Sync + 'static>(
    matchers: Vec<Arc<dyn TypedMatcher<T> + Send + Sync + 'static>>,
) -> Arc<dyn TypedMatcher<T> + Send + Sync + 'static> {
    Arc::new(And { matchers })
}

/// Matcher that combines multiple matchers with AND logic
///
/// # Examples
///
/// ```
/// use caramelo::{and, expect};
/// use caramelo::matchers::{contains};
///
/// expect("hello").to_match(and!(contains("ell"), contains("llo")));
/// ```
pub struct And<T: Send + Sync + 'static> {
    matchers: Vec<Arc<dyn TypedMatcher<T> + Send + Sync + 'static>>,
}

unsafe impl<T: Send + Sync + 'static> Send for And<T> {}
unsafe impl<T: Send + Sync + 'static> Sync for And<T> {}

impl<T: Send + Sync + 'static> And<T> {
    /// Creates a new And matcher with the given matchers
    pub fn new(matchers: Vec<Arc<dyn TypedMatcher<T> + Send + Sync + 'static>>) -> Self {
        And { matchers }
    }
}

impl<T> Matcher<T> for And<T>
where
    T: Send + Sync + 'static,
{
    fn matches(&self, value: &T) -> bool {
        self.matchers
            .iter()
            .all(|m| m.matches(value))
    }

    fn description(&self) -> String {
        self.matchers
            .iter()
            .map(|m| m.description())
            .collect::<Vec<_>>()
            .join(" and ")
    }
}

impl<T> TypedMatcher<T> for And<T>
where
    T: Send + Sync + 'static,
{
    fn matcher_type(&self) -> MatchType {
        self.matchers
            .first()
            .map(|m| TypedMatcher::matcher_type(m.as_ref()))
            .unwrap_or(To)
    }
}
