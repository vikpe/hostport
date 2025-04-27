#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

//! HostPort
pub mod validate;

use anyhow::Result;
use std::fmt::Display;
use thiserror::Error;

#[cfg(feature = "json")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents a host and port combination.
#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct HostPort {
    /// Hostname, network alias or IP address.
    host: String,

    /// Port number.
    port: u16,
}

impl HostPort {
    /// Creates a new `HostPort` instance.
    /// # Examples
    /// ```
    /// use hostport::HostPort;
    ///
    /// let hostport = HostPort::new("quake.se", 28000).unwrap();
    /// assert_eq!(hostport.host(), "quake.se");
    /// assert_eq!(hostport.port(), 28000);
    /// ```
    pub fn new(host: &str, port: u16) -> Result<HostPort, HostPortError> {
        if !validate::is_valid_host(host) {
            return Err(HostPortError::InvalidHost(host.to_string()));
        }
        Ok(Self {
            host: host.to_string(),
            port,
        })
    }

    /// Returns the host part of the `HostPort`.
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Returns the port part of the `HostPort`.
    pub fn port(&self) -> u16 {
        self.port
    }
}

/// Implements the `From` trait for converting a `HostPort` to a string.
/// # Examples
/// ```
/// use hostport::HostPort;
/// let hostport = HostPort::new("quake.se", 28000).unwrap();
/// assert_eq!(hostport.to_string(), "quake.se:28000");
/// ```
impl TryFrom<&str> for HostPort {
    type Error = HostPortError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (host, port_str) = value.split_once(':').ok_or(HostPortError::InvalidFormat)?;

        let port = port_str
            .parse::<u16>()
            .map_err(|_| HostPortError::InvalidPort(port_str.to_string()))?;
        HostPort::new(host, port)
    }
}

impl Display for HostPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

#[cfg(feature = "json")]
impl Serialize for HostPort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "json")]
impl<'de> Deserialize<'de> for HostPort {
    fn deserialize<D>(deserializer: D) -> Result<HostPort, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string_value = String::deserialize(deserializer)?;
        HostPort::try_from(string_value.as_str()).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum HostPortError {
    #[error("Invalid format, expected host:port")]
    InvalidFormat,

    #[error("Invalid host: {0}")]
    InvalidHost(String),

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
    use validate::is_valid_host;

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
            prop_assert_eq!(result.err(), Some(HostPortError::InvalidFormat));

            // Case 2: Invalid port (non-numeric)
            if !invalid_port.is_empty() {
                let input = format!("{}:{}", host.clone(), invalid_port.clone());
                let result = HostPort::try_from(input.as_str());
                prop_assert!(result.is_err());
                prop_assert!(matches!(result.err(), Some(HostPortError::InvalidPort(_))));
            }

            // Case 3: Invalid host with valid port
            let invalid_host = format!("{}$", host); // Add invalid character
            let input = format!("{}:{}", invalid_host, port);
            let result = HostPort::try_from(input.as_str());
            prop_assert!(result.is_err());
            prop_assert!(matches!(result.err(), Some(HostPortError::InvalidHost(_))));

            // Case 4: Empty string before colon
            let input = format!(":{}", port);
            let result = HostPort::try_from(input.as_str());
            prop_assert!(result.is_err());

            // Case 5: Nothing after colon
            let input = format!("{}:", host);
            let result = HostPort::try_from(input.as_str());
            prop_assert!(result.is_err());
            prop_assert!(matches!(result.err(), Some(HostPortError::InvalidPort(_))));
        }
    }

    #[test]
    fn test_new() -> Result<()> {
        {
            assert_eq!(
                HostPort::new("_", 50).unwrap_err(),
                HostPortError::InvalidHost("_".to_string())
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
    #[cfg(feature = "json")]
    fn test_serialize() -> Result<()> {
        let hostport = HostPort::new("quake.se", 28501)?;
        assert_eq!(
            serde_json::to_string(&hostport)?,
            r#""quake.se:28501""#.to_string(),
        );
        Ok(())
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_deserialize() -> Result<()> {
        assert_eq!(
            serde_json::from_str::<HostPort>(r#""quake.se:28501""#)?,
            HostPort {
                host: "quake.se".to_string(),
                port: 28501,
            }
        );
        assert!(serde_json::from_str::<HostPort>(r#"5"#).is_err());
        Ok(())
    }
}
