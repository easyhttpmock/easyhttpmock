use caramelo::{
    MatchType::{self, To},
    Matcher, TypedMatcher,
};
use std::sync::Arc;

/// Creates a matcher that matches values that satisfy any of the given matchers
///
/// # Examples
///
/// ```
/// use caramelo::{or, expect};
/// use caramelo::matchers::{contains};
///
/// expect("hello").to_match(or!(contains("ell"), contains("xyz")));
/// ```
pub fn or<T: Send + Sync + 'static>(
    matchers: Vec<Arc<dyn TypedMatcher<T> + Send + Sync + 'static>>,
) -> Arc<dyn TypedMatcher<T> + Send + Sync + 'static> {
    Arc::new(Or { matchers })
}

/// Matcher that combines multiple matchers with OR logic
///
/// # Examples
///
/// ```
/// use caramelo::{or, expect};
/// use caramelo::matchers::{contains};
///
/// expect("hello").to_match(or!(contains("ell"), contains("xyz")));
/// ```
pub struct Or<T: Send + Sync + 'static> {
    matchers: Vec<Arc<dyn TypedMatcher<T> + Send + Sync + 'static>>,
}

unsafe impl<T: Send + Sync + 'static> Send for Or<T> {}
unsafe impl<T: Send + Sync + 'static> Sync for Or<T> {}

impl<T: Send + Sync + 'static> Or<T> {
    /// Creates a new Or matcher with the given matchers
    pub fn new(matchers: Vec<Arc<dyn TypedMatcher<T> + Send + Sync + 'static>>) -> Self {
        Or { matchers }
    }
}

impl<T: Send + Sync + 'static> Matcher<T> for Or<T> {
    fn matches(&self, value: &T) -> bool {
        self.matchers
            .iter()
            .any(|m| m.matches(value))
    }

    fn description(&self) -> String {
        self.matchers
            .iter()
            .map(|m| m.description())
            .collect::<Vec<_>>()
            .join(" or ")
    }
}

impl<T: Send + Sync + 'static> TypedMatcher<T> for Or<T> {
    fn matcher_type(&self) -> MatchType {
        self.matchers
            .first()
            .map(|m| TypedMatcher::<T>::matcher_type(m.as_ref()))
            .unwrap_or(To)
    }
}
