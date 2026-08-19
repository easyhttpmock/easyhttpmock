use crate::vetis_adapter::{VetisAdapter, VetisAdapterConfig};
use deboa::{
    cert::{CertificateExt, ContentEncoding},
    request::get,
    HttpClient,
};
use deboa_compio::{cert::DeboaCertificate, Client};
use easyhttpmock::{
    config::EasyHttpMockConfig,
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
    server::PortGenerator,
    EasyHttpMock,
};
use http::{StatusCode, Version};
use std::error::Error;

const CA_CERT: &[u8] = include_bytes!("../../../certs/ca.der");
const SERVER_CERT: &[u8] = include_bytes!("../../../certs/server.der");
const SERVER_KEY: &[u8] = include_bytes!("../../../certs/server.key.der");

pub(crate) const fn default_protocol() -> Version {
    #[cfg(feature = "http1")]
    return Version::HTTP_11;
    #[cfg(feature = "http2")]
    return Version::HTTP_2;
    #[cfg(feature = "http3")]
    return Version::HTTP_3;
}

#[compio::test]
async fn test_mock_request() -> Result<(), Box<dyn Error>> {
    let server_cert = SERVER_CERT;
    let server_key = SERVER_KEY;

    let vetis_adapter_config = VetisAdapterConfig::builder()
        .protos(vec![default_protocol()])
        .with_random_port()
        .cert(server_cert.to_vec())
        .key(server_key.to_vec())
        .ca(CA_CERT.to_vec())
        .build();

    let config = EasyHttpMockConfig::<VetisAdapter>::builder()
        .server_config(vetis_adapter_config)
        .build();

    let Ok(mut server) = EasyHttpMock::new(config) else {
        panic!("Failed to create mock server");
    };

    let mock = Mock::of(
        given(path("/test").and(method("GET"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"teste"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let client = Client::builder()
        .certificate(DeboaCertificate::from_slice(CA_CERT, ContentEncoding::DER))
        .build();

    let request = get(server.url("/test"))?.build()?;
    let response = client
        .execute(request)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    server
        .stop()
        .await?;

    Ok(())
}
