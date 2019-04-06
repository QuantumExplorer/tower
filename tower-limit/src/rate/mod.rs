//! Limit the rate at which requests are processed.

pub mod error;
pub mod future;
mod layer;
mod rate;
mod service;

pub use self::layer::RateLimitLayer;
pub use self::rate::Rate;
pub use self::service::RateLimit;