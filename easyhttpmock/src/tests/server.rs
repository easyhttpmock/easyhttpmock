#![allow(dead_code)]
use crate::{
    config::EasyHttpMockConfig,
    errors::EasyHttpMockError,
    mock::Mock,
    server::{PortGenerator, ServerAdapter},
    tests::TestResult,
    EasyHttpMock,
};
use caramelo::{
    expect,
    matchers::{eq, ge, lt},
    MatcherExt,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TestServerConfig {
    port: u32,
    interface: String,
}

impl Default for TestServerConfig {
    fn default() -> Self {
        Self { port: 8080, interface: "127.0.0.1".to_string() }
    }
}

impl TestServerConfig {
    pub fn port(&self) -> u32 {
        self.port
    }

    pub fn interface(&self) -> &str {
        &self.interface
    }
}

pub struct TestServer {
    config: TestServerConfig,
    mock: Option<Arc<Mock>>,
}

impl Default for TestServer {
    fn default() -> Self {
        Self { config: TestServerConfig::default(), mock: None }
    }
}

impl ServerAdapter for TestServer {
    type Config = TestServerConfig;

    fn new(config: Self::Config) -> Result<Self, EasyHttpMockError> {
        Ok(Self { config, mock: None })
    }

    fn hostname(&self) -> String {
        "localhost".to_string()
    }

    fn base_url(&self) -> String {
        format!("http://{}:{}", self.hostname(), self.config.port)
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn config_mut(&mut self) -> &mut Self::Config {
        &mut self.config
    }

    fn register_mock(&mut self, mock: Arc<Mock>) {
        self.mock = Some(mock);
    }

    async fn start(&mut self) -> Result<(), EasyHttpMockError> {
        todo!()
    }

    async fn stop(&mut self) -> Result<(), EasyHttpMockError> {
        todo!()
    }
}

impl PortGenerator<TestServer> for TestServerConfig {
    fn with_random_port(self) -> Self {
        let port = rand::random_range(9000..65535);
        Self { port, ..self }
    }
}

#[test]
fn test_server() -> TestResult<()> {
    let mock_server = EasyHttpMock::<TestServer>::new(crate::config::EasyHttpMockConfig {
        server_config: TestServerConfig { port: 7070, interface: "127.0.0.1".to_string() },
        base_url: Some("http://localhost:7070".to_string()),
    })?;

    expect(mock_server.base_url()).to_be(eq("http://localhost:7070"));
    expect(mock_server.url("/users")).to_be(eq("http://localhost:7070/users"));
    expect(
        *&mock_server
            .config()
            .port(),
    )
    .to_be(eq(7070));

    assert_eq!(
        mock_server
            .config
            .server_config
            .port,
        7070,
        "server port should be 7070"
    );

    Ok(())
}

#[test]
fn test_random_port() -> TestResult<()> {
    let mock_server = EasyHttpMock::<TestServer>::new(EasyHttpMockConfig {
        server_config: TestServerConfig { port: 0, interface: "127.0.0.1".to_string() },
        base_url: None,
    })?;

    let config = mock_server
        .config
        .server_config
        .with_random_port();

    expect(config.port).to_be(ge(9000).and(lt(65535)));

    Ok(())
}

#[test]
fn test_server_config_builder() -> TestResult<()> {
    let server_config = TestServerConfig::default();
    let mock_server = EasyHttpMockConfig::<TestServer>::builder()
        .base_url("http://test.local".into())
        .server_config(server_config)
        .build();
    expect(mock_server.base_url()).to_be(eq(&Some("http://test.local".into())));
    expect(
        mock_server
            .server_config()
            .port(),
    )
    .to_be(eq(8080));
    Ok(())
}

#[test]
fn test_server_config_default() -> TestResult<()> {
    let mock_server = EasyHttpMockConfig::<TestServer>::default();
    expect(
        mock_server
            .server_config()
            .port(),
    )
    .to_be(eq(8080));
    Ok(())
}
