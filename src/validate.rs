//! Validation

/// Checks if the given string is a valid host
/// # Examples
/// ```
/// use hostport::validate::is_valid_host;
/// assert!(is_valid_host("quake.se"));
/// assert!(is_valid_host("quake-world.se"));
/// assert!(is_valid_host("localhost"));
/// assert!(is_valid_host("10.10.10.10"));
/// assert!(!is_valid_host("foo."));
pub fn is_valid_host(value: &str) -> bool {
    if value.is_empty() || value.len() > 255 {
        return false;
    }
    if value.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    value.split('.').all(is_valid_label)
}

fn is_valid_label(label: &str) -> bool {
    if label.is_empty() || label.len() > 63 {
        return false;
    }
    if label.starts_with('-') || label.ends_with('-') {
        return false;
    }
    if !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return false;
    }
    true
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
        assert!(!is_valid_label("a|b"));

        // valid
        assert!(is_valid_label("a"));
        assert!(is_valid_label("a-b"));
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
        assert!(!is_valid_host("0"));
        assert!(!is_valid_host("10"));
        assert!(!is_valid_host("quake|se"));

        // valid
        assert!(is_valid_host("quake1.se"));
        assert!(is_valid_host("1quake.se"));
    }
}
