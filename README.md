# EasyHttpMock

[![Crates.io downloads](https://img.shields.io/crates/d/easyhttpmock)](https://crates.io/crates/easyhttpmock) [![crates.io](https://img.shields.io/crates/v/easyhttpmock?style=flat-square)](https://crates.io/crates/easyhttpmock) [![Build Status](https://github.com/easyhttpmock/easyhttpmock/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/easyhttpmock/easyhttpmock/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/easyhttpmock) [![Documentation](https://docs.rs/easyhttpmock/badge.svg)](https://docs.rs/easyhttpmock/latest/easyhttpmock) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/easyhttpmock/easyhttpmock/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/easyhttpmock/easyhttpmock/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/easyhttpmock/easyhttpmock)

**EasyHttpMock** is a powerful yet simple HTTP mock server designed specifically for testing HTTP clients. Built to work with any web server, it provides a clean, intuitive API for creating realistic mock endpoints that simulate real-world API behavior, making your testing workflow faster and more reliable.

## Why EasyHttpMock?

- **Testing-Focused**: Purpose-built for HTTP client testing scenarios
- **Lightning Fast**: Powered by VeTiS for optimal performance
- **Flexible Runtime**: Choose between Tokio or Smol async runtimes
- **Full Protocol Support**: HTTP/1, HTTP/2, and HTTP/3 compatibility
- **Secure Testing**: Built-in TLS support for HTTPS endpoint testing
- **Minimal Dependencies**: Lightweight footprint for your test suite

## Quick Start

Add EasyHttpMock to your `Cargo.toml`:

```toml
easyhttpmock = { version = "0.1.1", features = ["tokio-rt", "http1"] }
```

## Usage Example

Here's how simple it is to create a mock HTTP server for testing:

```rust
use easyhttpmock_vetis_tokio::{
    EasyHttpMock,
    config::EasyHttpMockConfig,
    matchers::{method, path},
    mock::{AsyncMatcherExt, Mock, StatusCodeExt, given},
    vetis_adapter::VetisAdapter,
};
use http::{Method, StatusCode};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = EasyHttpMockConfig::<VetisAdapter>::default();
    let server = EasyHttpMock::new(config);
    let mut server = match server {
        Ok(server) => server,
        Err(err) => {
            panic!("Failed to create mock server: {}", err);
        }
    };

    let mock = Mock::of(
        given(method(Method::GET).and(path("/test"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"teste"),
        ),
    );

    server.register_mock(mock).await?;;

    Ok(())
}
```

## Perfect For

- **Unit Testing**: Mock external APIs in your test suite
- **Integration Testing**: Test HTTP client behavior without real services
- **Load Testing**: Simulate API responses under various conditions
- **Debugging**: Reproduce API issues in a controlled environment
- **Documentation**: Create interactive API examples

## Supported Runtimes (via crates)

- [tokio](https://github.com/tokio-rs/tokio) - High-performance async runtime
- [smol](https://github.com/smol-rs/smol) - Lightweight async runtime

## Crate Features

- **http1** - HTTP/1 protocol support
- **http2** (default) - HTTP/2 protocol support
- **http3** - HTTP/3 protocol support
- **rust-tls** (default) - TLS support

## Subprojects

### [easyhttpmock](https://github.com/easyhttpmock/easyhttpmock)

Base crate for easyhttpmock.

### [easyhttpmock-vetis-smol](https://github.com/easyhttpmock/easyhttpmock-vetis-smol)

Adapter for vetis using smol runtime.

### [easyhttpmock-vetis-tokio](https://github.com/easyhttpmock/easyhttpmock-vetis-tokio)

Adapter for vetis using tokio runtime.

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
