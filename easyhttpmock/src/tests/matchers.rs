use crate::{
    matchers::{
        basic_auth, body, header, header_value, jwt, method, path, query_param, query_value, And,
        AsMethod, Or,
    },
    mock::{AsyncMatcherExt, Request},
};
use bytes::Bytes;
use caramelo::{expect, matchers::eq, MatchType::ToHave, TypedMatcher};
use http::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Method, Uri, Version,
};

#[test]
fn test_path_matcher() {
    let request = Request::get(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(path(r"^/api/.*$").and(method("GET")));
}

#[test]
fn test_query_matcher() {
    let request = Request::get(Uri::from_static("/api/users?name=mark&age=31"))
        .empty()
        .unwrap();

    expect(request).to_have(
        path(r"^/api/.*$")
            .and(method("GET"))
            .and(query_param(r"^name$"))
            .and(query_value(r"^mark$")),
    );
}

#[test]
fn test_as_method_str() {
    let methods = ["POST", "DELETE", "PUT", "OPTIONS", "QUERY", "HEAD", "PATCH"];
    for method_name in methods {
        let uri = Uri::from_static("/api/users");
        let request = Request::builder(method_name.into_method(), uri)
            .empty()
            .unwrap();
        expect(request).to_have(method(method_name));
    }
}

#[test]
fn test_as_method_string() {
    let methods = ["POST".to_owned()];
    for method_name in methods {
        let uri = Uri::from_static("/api/users");
        let request = Request::builder(
            method_name
                .clone()
                .into_method(),
            uri,
        )
        .empty()
        .unwrap();
        expect(request).to_have(method(method_name));
    }
}

#[test]
fn test_new_or_matcher() {
    let or_matcher = Or::new(vec![method(Method::GET).into()]);
    expect(or_matcher.matcher_type()).to_be(eq(ToHave));
}

#[test]
fn test_new_and_matcher() {
    let and_matcher = And::new(vec![method(Method::GET).into()]);
    expect(and_matcher.matcher_type()).to_be(eq(ToHave));
}

#[test]
#[should_panic = "Expected Request { method: GET, uri: /users, version: HTTP/1.1, headers: {}, query_params: None, body: None } to have path matching Regex(\"^/api/.*$\")"]
fn test_path_matcher_panic() {
    let request = Request::get(Uri::from_static("/users"))
        .empty()
        .unwrap();

    expect(request).to_have(path(r"^/api/.*$").and(method(Method::GET)));
}

#[test]
fn test_method_get_matcher() {
    let request = Request::get(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::GET));
}

#[test]
fn test_method_post_matcher() {
    let request = Request::post(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::POST));
}

#[test]
fn test_method_put_matcher() {
    let request = Request::put(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::PUT));
}

#[test]
fn test_method_delete_matcher() {
    let request = Request::delete(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::DELETE));
}

#[test]
fn test_method_patch_matcher() {
    let request = Request::patch(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::PATCH));
}

#[test]
fn test_method_options_matcher() {
    let request = Request::options(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::OPTIONS));
}

#[test]
fn test_method_trace_matcher() {
    let request = Request::trace(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::TRACE));
}

#[test]
fn test_method_connect_matcher() {
    let request = Request::connect(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::CONNECT));
}

#[test]
#[should_panic = "Expected Request { method: POST, uri: /api/users, version: HTTP/1.1, headers: {}, query_params: None, body: None } to have method matching GET"]
fn test_method_matcher_panic() {
    let request = Request::post(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method("GET"));
}

#[test]
#[should_panic = "Expected Request { method: POST, uri: /api/users, version: HTTP/1.1, headers: {}, query_params: None, body: None } to have method matching POST and path matching Regex(\"^/api/posts$\")"]
fn test_method_and_path_matcher_panic() {
    let request = Request::post(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::POST).and(path(r"^/api/posts$")));
}

#[test]
fn test_header_matcher() {
    let request = Request::get(Uri::from_static("/api/users"))
        .header("content-type", "application/json")
        .empty()
        .unwrap();

    expect(request).to_have(header(CONTENT_TYPE));
}

#[test]
fn test_header_jwt_matcher() {
    let request = Request::get(Uri::from_static("/api/users"))
        .header(AUTHORIZATION, "Bearer 123456")
        .empty()
        .unwrap();

    expect(request).to_have(jwt("123456"));
}

#[test]
fn test_header_basic_auth_matcher() {
    let request = Request::get(Uri::from_static("/api/users"))
        .header(AUTHORIZATION, "Basic bWFyazpqb2huc29u")
        .empty()
        .unwrap();

    expect(request).to_have(basic_auth("mark", "johnson"));
}

#[test]
fn test_version_matcher() {
    let request = Request::get(Uri::from_static("/api/users"))
        .version(Version::HTTP_11)
        .empty()
        .unwrap();

    expect(request.version()).to_be(eq(&Version::HTTP_11));
}

#[test]
fn test_req_method_matcher() {
    let request = Request::get(Uri::from_static("/api/users"))
        .method(Method::GET)
        .empty()
        .unwrap();

    expect(request.method()).to_be(eq(&Method::GET));
}

#[test]
#[should_panic = "Expected Request { method: GET, uri: /api/users, version: HTTP/1.1, headers: {\"content-store\": \"application/json\"}, query_params: None, body: None } to have header matching content-type"]
fn test_header_matcher_failure() {
    let request = Request::get(Uri::from_static("/api/users"))
        .header("content-store", "application/json")
        .empty()
        .unwrap();

    expect(request).to_have(header(CONTENT_TYPE));
}

#[test]
fn test_header_value_regex() {
    let request = Request::get(Uri::from_static("/api/users"))
        .header("content-type", "application/json")
        .empty()
        .unwrap();

    expect(request).to_have(header_value("content-type", r"^application/.*"));
}

#[test]
#[should_panic = "Expected Request { method: GET, uri: /api/users, version: HTTP/1.1, headers: {\"content-type\": \"application/json\"}, query_params: None, body: None } to have header content-type with value matching Regex(\"^text/.*\")"]
fn test_header_value_regex_failure() {
    let request = Request::get(Uri::from_static("/api/users"))
        .header("content-type", "application/json")
        .empty()
        .unwrap();

    expect(request).to_have(header_value("content-type", r"^text/.*"));
}

#[test]
fn test_body() {
    let request = Request::get(Uri::from_static("/api/users"))
        .header("content-type", "application/json")
        .body(Bytes::from_static(b"Hello world!"))
        .unwrap();

    expect(request).to_have(body(r".*wor.*d!"));
}

#[test]
#[should_panic = "Expected Request { method: GET, uri: /api/users, version: HTTP/1.1, headers: {\"content-type\": \"application/json\"}, query_params: None, body: Some(b\"Hello world!\") } to have body contents matching Regex(\".*war.*d!\")"]
fn test_body_failure() {
    let request = Request::get(Uri::from_static("/api/users"))
        .header("content-type", "application/json")
        .body(Bytes::from_static(b"Hello world!"))
        .unwrap();

    expect(request).to_have(body(r".*war.*d!"));
}

#[cfg(feature = "json")]
#[cfg(test)]
mod json_test {
    use crate::{
        matchers::{exact_json_body, partial_json_body},
        mock::Request,
    };
    use bytes::Bytes;
    use caramelo::expect;
    use http::Uri;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Serialize, Deserialize)]
    struct Payload {
        code: u32,
        message: String,
    }

    #[test]
    fn test_partial_json() {
        let data = json!({ "code": 200, "message": "Something went wrong" });
        let request = Request::get(Uri::from_static("/api/users"))
            .header("content-type", "application/json")
            .body(Bytes::copy_from_slice(
                data.to_string()
                    .as_bytes(),
            ))
            .unwrap();
        expect(request).to_have(partial_json_body(r#"$.code"#));
    }

    #[test]
    #[should_panic = "Expected Request { method: GET, uri: /api/users, version: HTTP/1.1, headers: {\"content-type\": \"application/json\"}, query_params: None, body: Some(b\"{\\\"code\\\":200,\\\"message\\\":\\\"Something went wrong\\\"}\") } to have body contents containing $.name"]
    fn test_partial_json_failure() {
        let data = json!({ "code": 200, "message": "Something went wrong" });
        let request = Request::get(Uri::from_static("/api/users"))
            .header("content-type", "application/json")
            .body(Bytes::copy_from_slice(
                data.to_string()
                    .as_bytes(),
            ))
            .unwrap();
        expect(request).to_have(partial_json_body(r#"$.name"#));
    }

    #[test]
    fn test_exact_json() {
        let data = json!({ "code": 200, "message": "Something went wrong" });
        let request = Request::get(Uri::from_static("/api/users"))
            .header("content-type", "application/json")
            .body(Bytes::copy_from_slice(
                data.to_string()
                    .as_bytes(),
            ))
            .unwrap();
        let payload = Payload { code: 200, message: "Something went wrong".into() };
        expect(request).to_have(exact_json_body(&payload));
    }

    #[test]
    #[should_panic = "xpected Request { method: GET, uri: /api/users, version: HTTP/1.1, headers: {\"content-type\": \"application/json\"}, query_params: None, body: Some(b\"{\\\"code\\\":200,\\\"message\\\":\\\"Something went wrong\\\"}\") } to have body contents matching {\"code\":500,\"message\":\"Something went wrong\"}"]
    fn test_exact_json_failure() {
        let data = json!({ "code": 200, "message": "Something went wrong" });
        let request = Request::get(Uri::from_static("/api/users"))
            .header("content-type", "application/json")
            .body(Bytes::copy_from_slice(
                data.to_string()
                    .as_bytes(),
            ))
            .unwrap();
        let payload = Payload { code: 500, message: "Something went wrong".into() };
        expect(request).to_have(exact_json_body(&payload));
    }
}

#[cfg(feature = "xml")]
#[cfg(test)]
mod xml_test {
    use crate::{
        matchers::{exact_xml_body, partial_xml_body},
        mock::Request,
    };
    use bytes::Bytes;
    use caramelo::expect;
    use http::Uri;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct Person {
        id: u32,
        name: String,
    }

    #[test]
    fn test_partial_xml() {
        let data = "<response><code>200</code><message>Something went wrong</message></response>";
        let request = Request::get(Uri::from_static("/api/users"))
            .header("content-type", "application/xml")
            .body(Bytes::copy_from_slice(
                data.to_string()
                    .as_bytes(),
            ))
            .unwrap();
        expect(request).to_have(partial_xml_body(r#"//response/code"#));
    }

    #[test]
    #[should_panic = "Expected Request { method: GET, uri: /api/users, version: HTTP/1.1, headers: {\"content-type\": \"application/xml\"}, query_params: None, body: Some(b\"<response><code>200</code><message>Something went wrong</message></response>\") } to have body contents containing //response/name"]
    fn test_partial_xml_failure() {
        let data = "<response><code>200</code><message>Something went wrong</message></response>";
        let request = Request::get(Uri::from_static("/api/users"))
            .header("content-type", "application/xml")
            .body(Bytes::copy_from_slice(
                data.to_string()
                    .as_bytes(),
            ))
            .unwrap();
        expect(request).to_have(partial_xml_body(r#"//response/name"#));
    }

    #[test]
    fn test_exact_xml() {
        let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Person><id>12</id><name>take easy</name></Person>";
        let request = Request::get(Uri::from_static("/api/users"))
            .header("content-type", "application/xml")
            .body(Bytes::copy_from_slice(
                data.to_string()
                    .as_bytes(),
            ))
            .unwrap();
        let person = Person { id: 12, name: "take easy".into() };
        expect(request).to_have(exact_xml_body(&person));
    }

    #[test]
    #[should_panic = "Expected Request { method: GET, uri: /api/users, version: HTTP/1.1, headers: {\"content-type\": \"application/xml\"}, query_params: None, body: Some(b\"<Person><id>12</id><name>take easy</name></response>\") } to have body contents matching <?xml version=\"1.0\" encoding=\"UTF-8\"?><Person><id>32</id><name>take easy</name></Person>"]
    fn test_exact_xml_failure() {
        let data = "<Person><id>12</id><name>take easy</name></response>";
        let request = Request::get(Uri::from_static("/api/users"))
            .header("content-type", "application/xml")
            .body(Bytes::copy_from_slice(
                data.to_string()
                    .as_bytes(),
            ))
            .unwrap();
        let person = Person { id: 32, name: "take easy".into() };
        expect(request).to_have(exact_xml_body(&person));
    }
}
