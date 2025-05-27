use crate::is_valid_host;
use std::fmt::Display;
use std::net::SocketAddrV4;
use std::str::FromStr;
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a host and port combination.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HostPort {
    /// Hostname, network alias, or IP address.
    host: String,

    /// Port number.
    port: u16,
}

impl HostPort {
    /// Creates a new `HostPort` instance.
    ///
    /// # Examples
    /// ```
    /// use hostport::HostPort;
    ///
    /// let hostport = HostPort::new("localhost", 8080).unwrap();
    /// assert_eq!(hostport.host(), "localhost");
    /// assert_eq!(hostport.port(), 8080);
    /// assert_eq!(hostport.to_string(), "localhost:8080");
    /// ```
    pub fn new<S: Into<String>>(host: S, port: u16) -> Result<HostPort, ParseError> {
        let host = host.into();
        if !is_valid_host(&host) {
            return Err(ParseError::InvalidHost(host));
        }
        Ok(Self { host, port })
    }

    /// Returns the host part of the `HostPort`.
    #[must_use]
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Returns the port part of the `HostPort`.
    #[must_use]
    pub fn port(&self) -> u16 {
        self.port
    }
}

/// Implements the `From` trait for converting a `HostPort` to a string.
///
/// # Examples
/// ```
/// use hostport::HostPort;
///
/// let domain = HostPort::try_from("quake.se:28000").unwrap();
/// assert_eq!(domain.host(), "quake.se");
/// assert_eq!(domain.port(), 28000);
///
/// let ip = HostPort::try_from("10.10.10.10:28000").unwrap();
/// assert_eq!(ip.host(), "10.10.10.10");
/// assert_eq!(ip.port(), 28000);
///
/// let network_alias = HostPort::try_from("localhost:28000").unwrap();
/// assert_eq!(network_alias.host(), "localhost");
/// assert_eq!(network_alias.port(), 28000);
/// ```
impl TryFrom<&str> for HostPort {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (host, port_str) = value.split_once(':').ok_or(ParseError::InvalidFormat)?;

        let port = port_str
            .parse::<u16>()
            .map_err(|_| ParseError::InvalidPort(port_str.to_string()))?;
        HostPort::new(host, port)
    }
}

impl Display for HostPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

impl From<&SocketAddrV4> for HostPort {
    fn from(socket_addr: &SocketAddrV4) -> Self {
        HostPort {
            host: socket_addr.ip().to_string(),
            port: socket_addr.port(),
        }
    }
}

impl From<SocketAddrV4> for HostPort {
    fn from(socket_addr: SocketAddrV4) -> Self {
        Self::from(&socket_addr)
    }
}

impl FromStr for HostPort {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        HostPort::try_from(value)
    }
}

impl PartialEq<&str> for HostPort {
    fn eq(&self, other: &&str) -> bool {
        HostPort::try_from(*other)
            .map(|hp| self == &hp)
            .unwrap_or(false)
    }
}

impl PartialEq<HostPort> for &str {
    fn eq(&self, other: &HostPort) -> bool {
        HostPort::try_from(*self)
            .map(|hp| &hp == other)
            .unwrap_or(false)
    }
}

/// Errors that can occur while parsing a [`HostPort`].
///
/// # Variants
/// - `InvalidFormat`: The input string does not follow the `host:port` format.
/// - `InvalidHost`: The host part of the input is invalid.
/// - `InvalidPort`: The port part of the input is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum ParseError {
    /// The input string does not follow the `host:port` format.
    #[error("Invalid format, expected host:port")]
    InvalidFormat,

    /// The host part of the input is invalid.
    #[error("Invalid host: {0}")]
    InvalidHost(String),

    /// The port part of the input is invalid.
    #[error("Invalid port: {0}")]
    InvalidPort(String),
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn hostport_try_from_proptest(
            // Generate valid hostnames
            host in r"[a-zA-Z0-9]([-a-zA-Z0-9]*[a-zA-Z0-9])?((\.[a-zA-Z0-9]([-a-zA-Z0-9]*[a-zA-Z0-9])?)*)?",
            port in any::<u16>(),
            // Generate invalid inputs for negative testing
            invalid_str in r"[^:]*",
            invalid_port in r"[a-zA-Z]{1,10}"
        ) {
            // Skip valid hosts
            if !is_valid_host(&host) {
                return Ok(());
            }

            // Valid case: host:port should parse correctly
            let input = format!("{}:{}", &host, &port);
            let result = HostPort::try_from(input.as_str());

            prop_assert!(result.is_ok());
            let hostport = result.unwrap();
            prop_assert_eq!(hostport.host(), host.clone());
            prop_assert_eq!(hostport.port(), port);

            // Invalid cases

            // Case 1: No colon separator
            let result = HostPort::try_from(invalid_str.as_str());
            prop_assert_eq!(result.err(), Some(ParseError::InvalidFormat));

            // Case 2: Invalid port (non-numeric)
            if !invalid_port.is_empty() {
                let input = format!("{}:{}", host.clone(), invalid_port.clone());
                let result = HostPort::try_from(input.as_str());
                prop_assert!(result.is_err());
                prop_assert!(matches!(result.err(), Some(ParseError::InvalidPort(_))));
            }

            // Case 3: Invalid host with valid port
            let invalid_host = format!("{}$", host); // Add invalid character
            let input = format!("{}:{}", invalid_host, port);
            let result = HostPort::try_from(input.as_str());
            prop_assert!(result.is_err());
            prop_assert!(matches!(result.err(), Some(ParseError::InvalidHost(_))));

            // Case 4: Empty string before colon
            let input = format!(":{}", port);
            let result = HostPort::try_from(input.as_str());
            prop_assert!(result.is_err());

            // Case 5: Nothing after colon
            let input = format!("{}:", host);
            let result = HostPort::try_from(input.as_str());
            prop_assert!(result.is_err());
            prop_assert!(matches!(result.err(), Some(ParseError::InvalidPort(_))));
        }
    }

    #[test]
    fn test_new() -> Result<()> {
        {
            assert_eq!(
                HostPort::new("_", 50).unwrap_err(),
                ParseError::InvalidHost("_".to_string())
            );
        }
        {
            let hostport = HostPort::new("quake.se", 28501)?;
            assert_eq!(hostport.host(), "quake.se");
            assert_eq!(hostport.port(), 28501);
        }
        Ok(())
    }

    #[test]
    fn test_display() -> Result<()> {
        let hostport = HostPort::new("quake.se", 28501)?;
        assert_eq!(hostport.to_string(), "quake.se:28501");
        Ok(())
    }

    #[test]
    fn test_from_socket_addr() -> Result<()> {
        let socket_addr = SocketAddrV4::from_str("10.10.10.10:28501")?;
        let hostport: HostPort = HostPort::from(&socket_addr);
        assert_eq!(hostport.host(), "10.10.10.10");
        assert_eq!(hostport.port(), 28501);
        assert_eq!(HostPort::from(&socket_addr), HostPort::from(socket_addr));
        Ok(())
    }

    #[test]
    fn test_from_str() -> Result<()> {
        let hostport = HostPort::from_str("quake.se:28501")?;
        assert_eq!(hostport.host(), "quake.se");
        assert_eq!(hostport.port(), 28501);
        Ok(())
    }

    #[test]
    fn test_partial_eq() -> Result<()> {
        assert_eq!(HostPort::new("quake.se", 28501)?, "quake.se:28501");
        assert_ne!(HostPort::new("quake.se", 28501)?, "quake.se:28502");

        assert_eq!("quake.se:28501", HostPort::new("quake.se", 28501)?);
        assert_ne!("quake.se:28502", HostPort::new("quake.se", 28501)?);
        Ok(())
    }
}
