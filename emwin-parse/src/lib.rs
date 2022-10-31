use async_trait::async_trait;
use header::AWIPSProductIdentifer;

pub mod dt;
pub mod header;

/// Any type implementing `ProductParser` contains all state needed to fallibly parse a `Product` from a
/// type implementing `AsyncRead`
#[async_trait]
pub trait ProductParser {
    type Error;
    type Product;

    /// Parse an instance of [Self::Product] from the given reader, optionally returning [Self::Error]
    /// should the given document be invalid
    async fn parse<R: tokio::io::AsyncBufRead>(
        &self,
        mut file: R,
        header: AWIPSProductIdentifer,
    ) -> Result<Self::Product, Self::Error>;
}

/// Context containing all state needed when parsing and operating on EMWIN data
#[derive(Clone, Debug)]
pub struct Context;

impl Context {
    /// Create a new `Context`
    pub fn new() -> Self {
        Self
    }
}
