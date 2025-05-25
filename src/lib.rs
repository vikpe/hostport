#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

//! # hostport
//!
//! A library for parsing, validating, and working with `host:port` combinations.
//!
//! ## Example
//! ```
//! use hostport::HostPort;
//!
//! let hostport = HostPort::new("localhost", 8080).unwrap();
//! assert_eq!(hostport.host(), "localhost");
//! assert_eq!(hostport.port(), 8080);
//!
//! assert_eq!(hostport.to_string(), "localhost:8080");
//! assert_eq!(hostport, "localhost:8080");
//! assert_eq!(hostport, "localhost:8080".parse::<HostPort>().unwrap());
//! ```

mod hostport;
mod validate;

pub use hostport::{HostPort, HostPortParseError};
pub use validate::is_valid_host;
