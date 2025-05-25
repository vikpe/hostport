# hostport [![Test](https://github.com/vikpe/hostport/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/hostport/actions/workflows/test.yml) [![codecov](https://codecov.io/gh/vikpe/hostport/graph/badge.svg?token=KwNnQ0ICcS)](https://codecov.io/gh/vikpe/hostport) [![crates](https://img.shields.io/crates/v/hostport)](https://crates.io/crates/hostport) [![docs.rs](https://img.shields.io/docsrs/hostport)](https://docs.rs/hostport/)

> A crate for working with host:port combinations

- `host` - Domain, network alias or IP.
- `port` - Port number in range `0-65535`.

## HostPort struct

```rust
use hostport::HostPort;

let hostport = HostPort::new("localhost", 8080).unwrap();
assert_eq!(hostport.host(), "localhost");
assert_eq!(hostport.port(), 8080);
assert_eq!(hostport.to_string(), "localhost:8080");

assert_eq!(hostport, "localhost:8080");
assert_eq!(hostport, "localhost:8080".parse::<HostPort>().unwrap());
```

## Validation

```rust
use hostport::is_valid_host;

assert!(is_valid_host("quake.se"));
assert!(is_valid_host("quake-world.se"));
assert!(is_valid_host("localhost"));
assert!(is_valid_host("10.10.10.10"));
```
