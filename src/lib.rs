#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

//! # hostport
//!
//! A library for parsing, validating, and working with `host:port` combinations, such as `localhost:8080`.
//!
//! * `host` - Domain, network alias or IP.
//! * `port` - Port number in range 0-65535.
//!
//! [`HostPort`] supports parsing from strings, formatting to strings, and direct comparison with string literals.
//!
//! ## Example
//! ```
//! use hostport::HostPort;
//!
//! let hostport = HostPort::new("localhost", 8080).unwrap();
//! assert_eq!(hostport.host(), "localhost");
//! assert_eq!(hostport.port(), 8080);
//! assert_eq!(hostport.to_string(), "localhost:8080");
//!
//! assert_eq!(hostport, "localhost:8080");
//! assert_eq!(hostport, "localhost:8080".parse::<HostPort>().unwrap());
//! ```

mod hostport;
mod validate;

pub use hostport::{HostPort, ParseError};
pub use validate::is_valid_host;
