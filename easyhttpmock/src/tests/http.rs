use crate::{
    matchers::{header, header_value, method, path},
    mock::Request,
};
use caramelo::{expect, matchers::eq, MatcherExt};
use http::{header::CONTENT_TYPE, Method, Uri, Version};

#[test]
fn test_path_matcher() {
    let request = Request::get(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(path(r"^/api/.*$").and(method("GET")));
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
