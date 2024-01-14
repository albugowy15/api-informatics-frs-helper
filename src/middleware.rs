use std::time::Duration;

use hyper::Method;
use tower::limit::RateLimitLayer;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
};

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any)
}

pub fn request_timeout() -> TimeoutLayer {
    TimeoutLayer::new(Duration::from_secs(10))
}

pub fn rate_limit() -> RateLimitLayer {
    RateLimitLayer::new(5, Duration::from_secs(1))
}
