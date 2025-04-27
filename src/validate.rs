//! Validation

use std::net::Ipv4Addr;

/// Checks if the given string is a valid host (network alias, domain or IP)
/// # Examples
/// ```
/// use hostport::validate::is_valid_host;
/// assert!(is_valid_host("quake.se"));
/// assert!(is_valid_host("quake-world.se"));
/// assert!(is_valid_host("localhost"));
/// assert!(is_valid_host("10.10.10.10"));
/// assert!(!is_valid_host("foo."));
/// assert!(!is_valid_host("a.0"));
/// assert!(!is_valid_host("1000.0.0.0"));
pub fn is_valid_host(value: &str) -> bool {
    if value.is_empty() || value.len() > 255 {
        return false;
    }

    if !value.chars().next().unwrap().is_ascii_alphanumeric() {
        return false;
    }

    if !value.chars().last().unwrap().is_ascii_alphanumeric() {
        return false;
    }

    if value.chars().any(|c| !is_valid_host_char(c)) {
        return false;
    }

    let parts: Vec<&str> = value.split('.').collect();

    match parts.len() {
        3 if value.chars().all(|c| c.is_ascii_digit() || c == '.') => {
            value.parse::<Ipv4Addr>().is_ok()
        }
        _ => parts.into_iter().all(is_valid_label),
    }
}

fn is_valid_label(label: &str) -> bool {
    if label.is_empty() || label.len() > 63 {
        return false;
    }
    if label.starts_with('-') || label.ends_with('-') {
        return false;
    }
    if label.chars().any(|c| !is_valid_label_char(c)) {
        return false;
    }
    if label.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    true
}

fn is_valid_host_char(c: char) -> bool {
    is_valid_label_char(c) || c == '.'
}

fn is_valid_label_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-'
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_label() {
        // invalid
        assert!(!is_valid_label(""));
        assert!(!is_valid_label("a".repeat(64).as_str()));
        assert!(!is_valid_label("a-"));
        assert!(!is_valid_label("-a"));
        assert!(!is_valid_label("-a-"));
        assert!(!is_valid_label("a|b"));
        assert!(!is_valid_label("000"));

        // valid
        assert!(is_valid_label("a"));
        assert!(is_valid_label("a-b"));
        assert!(is_valid_label("a-1"));
        assert!(is_valid_label("a1"));
        assert!(is_valid_label("a1b"));
        assert!(is_valid_label("a1-b2"));
    }

    #[test]
    fn test_is_valid_host() {
        // invalid
        assert!(!is_valid_host(""));
        assert!(!is_valid_host("a".repeat(256).as_str()));
        assert!(!is_valid_host("000"));
        assert!(!is_valid_host("---"));
        assert!(!is_valid_host("aaa-"));
        assert!(!is_valid_host("-aaa"));
        assert!(!is_valid_host("aaa."));
        assert!(!is_valid_host(".aaa"));
        assert!(!is_valid_host("a.0"));
        assert!(!is_valid_host("f%%"));
        assert!(!is_valid_host("0"));
        assert!(!is_valid_host("10"));
        assert!(!is_valid_host("quake|se"));

        // valid
        assert!(is_valid_host("quake1.se"));
        assert!(is_valid_host("1quake.se"));
    }
}
