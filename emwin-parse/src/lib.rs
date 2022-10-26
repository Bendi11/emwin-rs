use async_trait::async_trait;

pub mod dt;

/// Any type implementing `ProductParser` contains all state needed to fallibly parse a `Product` from a
/// type implementing `AsyncRead`
#[async_trait]
pub trait ProductParser {
    type Error;
    type Product;
    
    /// Parse an instance of [Self::Product] from the given reader, optionally returning [Self::Error]
    /// should the given document be invalid
    async fn parse<R: tokio::io::AsyncRead>(&self, file: R) -> Result<Self::Product, Self::Error>;
}
