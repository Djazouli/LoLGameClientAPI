pub mod model;
pub mod api;

// Re-expose our reqwest because we need a `Client` in `GameClient::new`
pub extern crate reqwest;