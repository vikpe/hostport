# hostport

> A crate for working with host:port combinations

* `host` - Domain, network alias or IP.
* `port` - Port number in range `0-65535`.

## HostPort struct

```rust
use hostport::HostPort;

let hostport = HostPort::new("quake.se", 28000)?;
assert_eq!(hostport.host(), "quake.se");
assert_eq!(hostport.port(), 28000);

let hostport = HostPort::try_from("quake.se:28000")?;
assert_eq!(hostport.host(), "quake.se");
assert_eq!(hostport.port(), 28000);
```

## Validation

```rust
use hostport::validate;

assert!(is_valid_host("quake.se"));
assert!(is_valid_host("quake-world.se"));
assert!(is_valid_host("localhost"));
assert!(is_valid_host("10.10.10.10"));
```