use crate::server::ServerAdapter;

/// A easyhttpmock configuration builder for server adapter
pub struct EasyHttpMockConfigBuilder<S>
where
    S: ServerAdapter,
{
    /// The base URL for the mock server
    base_url: Option<String>,
    /// The server configuration
    pub(crate) server_config: S::Config,
}

impl<S> EasyHttpMockConfigBuilder<S>
where
    S: ServerAdapter,
{
    /// Sets the base URL for the mock server
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL for the mock server
    ///
    /// # Returns
    ///
    /// * `Self` - The current instance
    ///
    pub fn base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    /// Sets the server configuration
    ///
    /// # Arguments
    ///
    /// * `server_config` - The server configuration
    ///
    /// # Returns
    ///
    /// * `Self` - The current instance
    ///
    pub fn server_config(mut self, server_config: S::Config) -> Self {
        self.server_config = server_config;
        self
    }

    /// Builds the configuration
    ///
    /// # Returns
    ///
    /// * `EasyHttpMockConfig<S>` - The built configuration
    ///
    pub fn build(self) -> EasyHttpMockConfig<S> {
        EasyHttpMockConfig { base_url: self.base_url, server_config: self.server_config }
    }
}

/// EasyHttpMock server adapter configuration
#[derive(Clone)]
pub struct EasyHttpMockConfig<S>
where
    S: ServerAdapter,
{
    /// The base URL for the mock server
    pub(crate) base_url: Option<String>,
    /// The server configuration
    pub(crate) server_config: S::Config,
}

impl<S> Default for EasyHttpMockConfig<S>
where
    S: ServerAdapter + Default,
    S::Config: Clone + Default,
{
    fn default() -> Self {
        Self { base_url: None, server_config: S::Config::default() }
    }
}

impl<S> EasyHttpMockConfig<S>
where
    S: ServerAdapter,
    S::Config: Clone + Default,
{
    /// Returns a new builder for the configuration
    ///
    /// # Returns
    ///
    /// * `EasyHttpMockConfigBuilder<S>` - A new builder for the configuration
    ///
    pub fn builder() -> EasyHttpMockConfigBuilder<S> {
        EasyHttpMockConfigBuilder { base_url: None, server_config: S::Config::default() }
    }

    /// Returns the base URL for the mock server
    ///
    /// # Returns
    ///
    /// * `&Option<String>` - The base URL for the mock server
    ///
    pub fn base_url(&self) -> &Option<String> {
        &self.base_url
    }

    /// Returns the server configuration
    ///
    /// # Returns
    ///
    /// * `&S::Config` - The server configuration
    ///
    pub fn server_config(&self) -> &S::Config {
        &self.server_config
    }
}
