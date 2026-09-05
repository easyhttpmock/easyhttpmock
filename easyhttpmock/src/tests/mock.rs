use crate::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, Request, Respond, StatusCodeExt as _},
};
use ::http::{header::CONTENT_TYPE, Request as HttpRequest, Version};
use bytes::Bytes;
use caramelo::{expect, matchers::eq, MatchType, Matcher, TypedMatcher};
use http::{Method, StatusCode, Uri};
use std::{collections::HashMap, sync::Arc};

#[test]
fn request_builder_creates_requests_and_exposes_all_fields() {
    let mut query_params = HashMap::new();
    query_params.insert("page".to_string(), "2".to_string());

    let request = Request::post(Uri::from_static("/users?page=1"))
        .uri(Uri::from_static("/users?page=2"))
        .query_params(query_params.clone())
        .method(Method::PUT)
        .version(Version::HTTP_2)
        .header(CONTENT_TYPE, "application/json")
        .empty()
        .unwrap();

    expect(request.path()).to_be(eq(&Uri::from_static("/users?page=2")));
    expect(request.method()).to_be(eq(&Method::PUT));
    expect(request.version()).to_be(eq(&Version::HTTP_2));
    expect(
        request.headers()[CONTENT_TYPE]
            .to_str()
            .unwrap(),
    )
    .to_be(eq("application/json"));
    expect(request.query_params()).to_be(eq(&Some(query_params)));
    expect(request.body()).to_be(eq(&None));

    let body_request = Request::get(Uri::from_static("/users"))
        .body()
        .unwrap();
    expect(body_request.body()).to_be(eq(&None));
}

#[test]
fn request_constructors_set_the_expected_methods() {
    let uri = Uri::from_static("/");
    let requests = [
        (
            Request::get(uri.clone())
                .empty()
                .unwrap(),
            Method::GET,
        ),
        (
            Request::post(uri.clone())
                .empty()
                .unwrap(),
            Method::POST,
        ),
        (
            Request::put(uri.clone())
                .empty()
                .unwrap(),
            Method::PUT,
        ),
        (
            Request::delete(uri.clone())
                .empty()
                .unwrap(),
            Method::DELETE,
        ),
        (
            Request::patch(uri.clone())
                .empty()
                .unwrap(),
            Method::PATCH,
        ),
        (
            Request::head(uri.clone())
                .empty()
                .unwrap(),
            Method::HEAD,
        ),
        (
            Request::options(uri.clone())
                .empty()
                .unwrap(),
            Method::OPTIONS,
        ),
        (
            Request::trace(uri.clone())
                .empty()
                .unwrap(),
            Method::TRACE,
        ),
        (
            Request::connect(uri)
                .empty()
                .unwrap(),
            Method::CONNECT,
        ),
    ];

    for (request, method) in requests {
        expect(request.method()).to_be(eq(&method));
        expect(request.version()).to_be(eq(&Version::HTTP_11));
    }
}

#[test]
fn request_from_parts_parses_query_parameters() {
    let (parts, _) = HttpRequest::builder()
        .method(Method::GET)
        .uri("/search?term=rust&page=2&ignored")
        .header("x-request-id", "42")
        .body(())
        .unwrap()
        .into_parts();

    let request = Request::from_parts(parts);
    let mut expected = HashMap::new();
    expected.insert("term".to_string(), "rust".to_string());
    expected.insert("page".to_string(), "2".to_string());

    expect(request.query_params()).to_be(eq(&Some(expected)));
    expect(
        request.headers()["x-request-id"]
            .to_str()
            .unwrap(),
    )
    .to_be(eq("42"));
    expect(request.body()).to_be(eq(&None));
}

#[test]
fn response_builders_create_empty_and_body_responses() {
    let response = StatusCode::CREATED
        .respond()
        .with_header("content-type", "text/plain")
        .with_headers(&[("x-first", "one"), ("x-second", "two")])
        .with_body(b"created");

    expect(response.status_code()).to_be(eq(StatusCode::CREATED));
    expect(response.headers()["content-type"].as_str()).to_be(eq("text/plain"));
    expect(response.headers()["x-first"].as_str()).to_be(eq("one"));
    expect(response.headers()["x-second"].as_str()).to_be(eq("two"));
    expect(response.body()).to_be(eq(Bytes::from_static(b"created")));

    let empty = Respond::builder()
        .with_status(StatusCode::NO_CONTENT)
        .empty();
    expect(empty.status_code()).to_be(eq(StatusCode::NO_CONTENT));
    expect(
        empty
            .body()
            .is_empty(),
    )
    .to_be(eq(true));

    let no_body = Respond::builder().no_body();
    expect(
        no_body
            .body()
            .is_empty(),
    )
    .to_be(eq(true));
}

#[test]
fn mock_types_store_matchers_and_responses() {
    let response = StatusCode::OK
        .respond()
        .with_body(b"ok");
    let request_mock = given(path("^/users$")).will_return(response.clone());
    expect(
        request_mock
            .respond()
            .is_some(),
    )
    .to_be(eq(true));

    let state = Mock::of(request_mock);
    let first_inner = state.inner();
    let second_inner = state.inner();

    expect(Arc::ptr_eq(&first_inner, &second_inner)).to_be(eq(true));
    expect(
        first_inner
            .request()
            .respond()
            .is_some(),
    )
    .to_be(eq(true));
    expect(
        first_inner
            .request()
            .respond(),
    )
    .to_be(eq(Some(&response)));
    expect(
        first_inner
            .request()
            .matcher()
            .matches(
                &Request::get(Uri::from_static("/users"))
                    .empty()
                    .unwrap(),
            ),
    )
    .to_be(eq(true));
}

#[test]
fn matcher_extensions_and_arc_delegation_work() {
    let matcher = AsyncMatcherExt::and(method(Method::GET), path("^/users$"));
    let request = Request::get(Uri::from_static("/users"))
        .empty()
        .unwrap();
    expect(matcher.matches(&request)).to_be(eq(true));

    let other_matcher = AsyncMatcherExt::or(method(Method::POST), path("^/users$"));
    expect(other_matcher.matches(&request)).to_be(eq(true));
    expect(
        !other_matcher
            .description()
            .is_empty(),
    )
    .to_be(eq(true));

    let delegated: Arc<dyn TypedMatcher<Request> + Send + Sync> = Arc::new(method(Method::GET));
    expect(delegated.matches(&request)).to_be(eq(true));
    expect(
        !delegated
            .description()
            .is_empty(),
    )
    .to_be(eq(true));
    expect(delegated.matcher_type()).to_be(eq(MatchType::ToHave));
}
