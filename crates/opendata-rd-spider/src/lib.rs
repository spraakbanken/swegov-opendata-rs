mod error;
mod item;
mod rd_spider;

pub use error::Error;
pub use item::Item;
pub use rd_spider::{RdSpider, RdSpiderOptions};

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
