use crate::mock::Request;
use caramelo::{MatchType::ToHave, Matcher, TypedMatcher};
use std::sync::Arc;

pub use self::json::*;
pub use self::xml::*;

/// Creates a matcher that checks if the request body matches the given regex pattern.
///
/// # Arguments
///
/// * `value` - The regex pattern to match against.
///
/// # Returns
///
/// * `Body` - A matcher that checks if the request body matches the given regex pattern.
///
/// # Panics
///
/// * Panics if the regex pattern is invalid.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::body;
///
/// let matcher = body(r"^Hello World$");
/// ```
pub fn body(value: &str) -> Arc<dyn TypedMatcher<Request> + Send + Sync + 'static> {
    let regex = regex::Regex::new(value);
    match regex {
        Ok(regex) => Arc::new(Body(regex)),
        Err(_) => panic!("Invalid regex pattern"),
    }
}

#[derive(Clone)]
/// A matcher that checks if the request body matches a regex pattern.
///
/// # Arguments
///
/// * `value` - The regex pattern to match against.
///
/// # Returns
///
/// * `Body` - A matcher that checks if the request body matches the given regex pattern.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::body;
///
/// let matcher = body(r"^Hello World$");
/// ```
pub struct Body(regex::Regex);

impl Matcher<Request> for Body {
    fn matches(&self, value: &Request) -> bool {
        if let Some(body) = &value.body() {
            self.0
                .is_match(&String::from_utf8_lossy(body))
        } else {
            false
        }
    }

    fn description(&self) -> String {
        format!("body contents matching {:?}", self.0)
    }
}

impl TypedMatcher<Request> for Body {
    fn matcher_type(&self) -> caramelo::MatchType {
        ToHave
    }
}

#[cfg(feature = "json")]
pub(crate) mod json {
    use std::sync::Arc;

    use caramelo::{MatchType::ToHave, Matcher, TypedMatcher};
    use jsonpath_rust::JsonPath;
    use sonic_rs::Serialize;

    use crate::mock::Request;

    /// Creates a matcher that checks if the request body matches the given JSON exactly.
    ///
    /// # Arguments
    ///
    /// * `value` - The JSON value to match against.
    ///
    /// # Returns
    ///
    /// * `BodyWithExactJson` - A matcher that checks if the request body matches the given JSON exactly.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use easyhttpmock::matchers::exact_json_body;
    ///
    /// let matcher = exact_json_body(&serde_json::json!({"name": "John", "age": 30}));
    /// ```
    pub fn exact_json_body<T: Serialize>(
        value: &T,
    ) -> Arc<dyn TypedMatcher<Request> + Send + Sync + 'static> {
        match sonic_rs::to_string(value) {
            Ok(json) => Arc::new(BodyWithExactJson(json)),
            Err(e) => panic!("Failed to serialize JSON: {}", e),
        }
    }

    #[derive(Clone)]
    /// A matcher that checks if the request body matches the given JSON exactly.
    ///
    /// # Arguments
    ///
    /// * `value` - The JSON value to match against.
    ///
    /// # Returns
    ///
    /// * `BodyWithExactJson` - A matcher that checks if the request body matches the given JSON exactly.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use easyhttpmock::matchers::exact_json_body;
    ///
    /// let matcher = exact_json_body(&serde_json::json!({"name": "John", "age": 30}));
    /// ```
    pub struct BodyWithExactJson(String);

    impl Matcher<Request> for BodyWithExactJson {
        fn matches(&self, value: &Request) -> bool {
            if let Some(body) = value.body() {
                body == &self.0
            } else {
                false
            }
        }

        fn description(&self) -> String {
            format!("body contents matching {}", self.0)
        }
    }

    impl TypedMatcher<Request> for BodyWithExactJson {
        fn matcher_type(&self) -> caramelo::MatchType {
            ToHave
        }
    }

    /// Creates a matcher that checks if the request body contains the given JSON partial.
    ///
    /// # Arguments
    ///
    /// * `value` - The JSON partial to match against.
    ///
    /// # Returns
    ///
    /// * `BodyWithPartialJson` - A matcher that checks if the request body contains the given JSON partial.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use easyhttpmock::matchers::partial_json_body;
    ///
    /// let matcher = partial_json_body(r#"$.name"#);
    /// ```
    pub fn partial_json_body(
        value: &str,
    ) -> Arc<dyn TypedMatcher<Request> + Send + Sync + 'static> {
        Arc::new(BodyWithPartialJson(value.to_owned()))
    }

    #[derive(Clone)]
    /// A matcher that checks if the request body contains the given JSON partial.
    ///
    /// # Arguments
    ///
    /// * `value` - The JSON partial to match against.
    ///
    /// # Returns
    ///
    /// * `BodyWithPartialJson` - A matcher that checks if the request body contains the given JSON partial.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use easyhttpmock::matchers::partial_json_body;
    ///
    /// let matcher = partial_json_body(r#"$.name"#);
    /// ```
    pub struct BodyWithPartialJson(String);

    impl Matcher<Request> for BodyWithPartialJson {
        fn matches(&self, value: &Request) -> bool {
            if let Some(body) = &value.body() {
                if let Ok(json) =
                    sonic_rs::from_str::<serde_json::Value>(&String::from_utf8_lossy(body))
                {
                    if let Ok(results) = json.query_with_path(self.0.as_str()) {
                        !results.is_empty()
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        }

        fn description(&self) -> String {
            format!("body contents containing {}", self.0)
        }
    }

    impl TypedMatcher<Request> for BodyWithPartialJson {
        fn matcher_type(&self) -> caramelo::MatchType {
            ToHave
        }
    }
}

#[cfg(feature = "xml")]
pub(crate) mod xml {
    use std::sync::Arc;

    use caramelo::{MatchType::ToHave, Matcher, TypedMatcher};
    use serde::Serialize;
    use serde_xml_rs::to_string;
    use simdxml::parse;

    use crate::mock::Request;

    /// Creates a matcher that checks if the request body matches the given XML exactly.
    ///
    /// # Arguments
    ///
    /// * `value` - The XML value to match against.
    ///
    /// # Returns
    ///
    /// * `BodyWithExactXml` - A matcher that checks if the request body matches the given XML exactly.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use easyhttpmock::matchers::exact_xml_body;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct User {
    ///     name: String,
    ///     age: u32,
    /// }
    ///
    /// let matcher = exact_xml_body(&User {
    ///     name: "John".to_string(),
    ///     age: 30,
    /// });
    /// ```
    pub fn exact_xml_body<T: Serialize>(
        value: &T,
    ) -> Arc<dyn TypedMatcher<Request> + Send + Sync + 'static> {
        match to_string(value) {
            Ok(xml) => Arc::new(BodyWithExactXml(xml)),
            Err(e) => panic!("Failed to serialize XML: {}", e),
        }
    }

    #[derive(Clone)]
    /// A matcher that checks if the request body matches the given XML exactly.
    ///
    /// # Arguments
    ///
    /// * `value` - The XML value to match against.
    ///
    /// # Returns
    ///
    /// * `BodyWithExactXml` - A matcher that checks if the request body matches the given XML exactly.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use easyhttpmock::matchers::exact_xml_body;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct User {
    ///     name: String,
    ///     age: u32,
    /// }
    ///
    /// let matcher = exact_xml_body(&User {
    ///     name: "John".to_string(),
    ///     age: 30,
    /// });
    /// ```
    pub struct BodyWithExactXml(String);

    impl Matcher<Request> for BodyWithExactXml {
        fn matches(&self, value: &Request) -> bool {
            if let Some(body) = value.body() {
                body == &self.0
            } else {
                false
            }
        }

        fn description(&self) -> String {
            format!("body contents matching {}", self.0)
        }
    }

    impl TypedMatcher<Request> for BodyWithExactXml {
        fn matcher_type(&self) -> caramelo::MatchType {
            ToHave
        }
    }

    /// Creates a matcher that checks if the request body contains the given XML partial.
    ///
    /// # Arguments
    ///
    /// * `value` - The XML partial to match against.
    ///
    /// # Returns
    ///
    /// * `BodyWithPartialXml` - A matcher that checks if the request body contains the given XML partial.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use easyhttpmock::matchers::partial_xml_body;
    ///
    /// let matcher = partial_xml_body(r#"//name"#);
    /// ```
    pub fn partial_xml_body(value: &str) -> Arc<dyn TypedMatcher<Request> + Send + Sync + 'static> {
        Arc::new(BodyWithPartialXml(value.to_owned()))
    }

    #[derive(Clone)]
    /// A matcher that checks if the request body contains the given XML partial.
    ///
    /// # Arguments
    ///
    /// * `value` - The XML partial to match against.
    ///
    /// # Returns
    ///
    /// * `BodyWithPartialXml` - A matcher that checks if the request body contains the given XML partial.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use easyhttpmock::matchers::partial_xml_body;
    ///
    /// let matcher = partial_xml_body(r#"//name"#);
    /// ```
    pub struct BodyWithPartialXml(String);

    impl Matcher<Request> for BodyWithPartialXml {
        fn matches(&self, value: &Request) -> bool {
            if let Some(body) = &value.body() {
                if let Ok(xml) = parse(body) {
                    if let Ok(results) = xml.xpath_string(self.0.as_str()) {
                        !results.is_empty()
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        }

        fn description(&self) -> String {
            format!("body contents containing {}", self.0)
        }
    }

    impl TypedMatcher<Request> for BodyWithPartialXml {
        fn matcher_type(&self) -> caramelo::MatchType {
            ToHave
        }
    }
}
