use std::error::Error;

mod http;
mod mock;
mod server;

pub(crate) type TestResult<T> = Result<T, Box<dyn Error>>;
