# hostport

> A crate for working with host:port combinations

```rust
use hostport::HostPort;

let hostport = HostPort::new("quake.se", 28000)?;
assert_eq!(hostport.host(), "quake.se");
assert_eq!(hostport.port(), 28000);

let hostport = HostPort::try_from("quake.se:28000")?;
assert_eq!(hostport.host(), "quake.se");
assert_eq!(hostport.port(), 28000);
```