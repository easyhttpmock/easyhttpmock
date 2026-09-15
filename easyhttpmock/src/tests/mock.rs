use crate::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, Request, Respond, StatusCodeExt as _},
    tests::TestResult,
};
use ::http::Request as HttpRequest;
use bytes::Bytes;
use caramelo::{
    expect,
    matchers::{contains_key, eq, truthy},
    MatchType::ToHave,
    Matcher,
};
use http::{Method, StatusCode, Uri};

#[test]
fn test_given() -> TestResult<()> {
    let (parts, _) = HttpRequest::builder()
        .method(Method::POST)
        .body(())?
        .into_parts();

    let request = Request::from_parts(parts, Bytes::default());
    let mock = given(method(Method::POST)).will_return(
        StatusCode::OK
            .respond()
            .no_body(),
    );
    expect(
        mock.matcher()
            .matches(&request),
    )
    .to_be(truthy());

    Ok(())
}

#[test]
fn test_given_or() -> TestResult<()> {
    let (parts, _) = HttpRequest::builder()
        .method(Method::POST)
        .body(())?
        .into_parts();

    let request = Request::from_parts(parts, Bytes::default());
    let mock = given(method(Method::POST).or(method(Method::PUT))).will_return(
        StatusCode::OK
            .respond()
            .no_body(),
    );
    expect(
        mock.matcher()
            .matches(&request),
    )
    .to_be(truthy());
    expect(
        mock.matcher()
            .matcher_type(),
    )
    .to_be(eq(ToHave));

    Ok(())
}

#[test]
fn test_given_and() -> TestResult<()> {
    let (parts, _) = HttpRequest::builder()
        .uri("/query")
        .method(Method::POST)
        .body(())?
        .into_parts();

    let request = Request::from_parts(parts, Bytes::default());
    let mock = given(method(Method::POST).and(path("/query"))).will_return(
        StatusCode::OK
            .respond()
            .no_body(),
    );
    expect(
        mock.matcher()
            .matches(&request),
    )
    .to_be(truthy());
    expect(
        mock.matcher()
            .matcher_type(),
    )
    .to_be(eq(ToHave));

    Ok(())
}

#[test]
fn test_mock_of() -> TestResult<()> {
    let (parts, _) = HttpRequest::builder()
        .method(Method::POST)
        .body(())?
        .into_parts();

    let request = Request::from_parts(parts, Bytes::default());
    let mock = Mock::of(
        given(method(Method::POST)).will_return(
            StatusCode::OK
                .respond()
                .no_body(),
        ),
    );

    expect(
        mock.inner()
            .request()
            .matcher()
            .matches(&request),
    )
    .to_be(truthy());

    Ok(())
}

#[test]
fn test_respond_builder() -> TestResult<()> {
    let respond = Respond::builder()
        .with_status(StatusCode::OK)
        .with_header("content-type", "application/json")
        .with_headers(&[("x-frame", "yes"), ("x-settings-yo", "again")])
        .empty();
    expect(respond.status_code()).to_be(eq(&StatusCode::OK));
    expect(respond.headers()["content-type"].as_str()).to_be(eq("application/json"));
    Ok(())
}

#[test]
fn test_request_builder() -> TestResult<()> {
    let uri = Uri::from_static("/users");
    let request = Request::builder(Method::GET, uri)
        .query_params([("q".into(), "search".into())].into())
        .body("test".into())?;

    expect(
        request
            .query_params()
            .clone()
            .unwrap(),
    )
    .to(contains_key("q"));

    Ok(())
}

#[test]
fn test_respond_builder_with_body() -> TestResult<()> {
    let respond = Respond::builder()
        .with_status(StatusCode::OK)
        .with_header("content-type", "application/json")
        .with_headers(&[("x-frame", "yes"), ("x-settings-yo", "again")])
        .with_body(b"test");
    expect(respond.status_code()).to_be(eq(&StatusCode::OK));
    expect(respond.headers()["content-type"].as_str()).to_be(eq("application/json"));
    expect(respond.body()).to_be(eq(&Bytes::from_static(b"test")));
    Ok(())
}
