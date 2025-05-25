# hostport [![Test](https://github.com/vikpe/hostport/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/hostport/actions/workflows/test.yml) [![codecov](https://codecov.io/gh/vikpe/hostport/graph/badge.svg?token=KwNnQ0ICcS)](https://codecov.io/gh/vikpe/hostport) [![crates.io](https://img.shields.io/crates/v/hostport)](https://crates.io/crates/hostport) [![docs.rs](https://img.shields.io/docsrs/hostport)](https://docs.rs/hostport/)

> A Rust crate for parsing, validating, and working with `host:port` combinations.

- **host**: Domain, network alias, or IP address
- **port**: Integer in the range `0–65535`

### Features

- Parse from strings
- Compare with strings
- Host validation

### Installation

```sh
cargo add hostport
```

### Usage

```rust
use hostport::HostPort;

let hostport = HostPort::new("localhost", 8080).unwrap();
assert_eq!(hostport.host(), "localhost");
assert_eq!(hostport.port(), 8080);
assert_eq!(hostport.to_string(), "localhost:8080");

assert_eq!(hostport, "localhost:8080");
assert_eq!(hostport, "localhost:8080".parse::<HostPort>().unwrap());
```

```rust
use hostport::is_valid_host;

assert!(is_valid_host("quake.se"));
assert!(is_valid_host("quake-world.se"));
assert!(is_valid_host("localhost"));
assert!(is_valid_host("10.10.10.10"));
```

### Optional Features

- **`serde`** – Enables serialization and deserialization support via [`serde`](https://crates.io/crates/serde).

```sh
cargo add hostport --features serde
```

or in `Cargo.toml`:

```toml
[dependencies]
hostport = { version = "x.y.z", features = ["serde"] }
```
