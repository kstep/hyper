//! Some HTTP context, like Request or Response, with common
//! features. Used to pass around some bits of information about
//! current execution context without making concrete assumptions
//! about its kind.

use url::{Url, UrlParser};
use header::Headers;

/// This is a HTTP trait
pub trait HttpContext {
    /// Base URL for a context
    fn base_url(&self) -> Option<&Url>;

    /// Context headers
    fn headers(&self) -> &Headers;

    /// URL parser preconfigured with base URL
    /// (if provided by the `base_url()` method above)
    fn url_parser(&self) -> UrlParser {
        let mut parser = UrlParser::new();
        if let Some(url) = self.base_url() {
            parser.base_url(url);
        }

        parser
    }
}
