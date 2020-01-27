# libbluetooth

**libbluetooth provides Rust bindings to Unix' Bluetooth API (BlueZ).**

## Usage

Add the following to your `cargo.toml`:

```toml
[target.'cfg(unix)'.dependencies]
libbluetooth = "0.2"
```

## No-std support

This crate does not require the Rust standard library.

## Platform support

libbluetooth is guaranteed to build for the following platforms:

 * x86_64-unknown-linux-gnu

## License

libbluetooth is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

 at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in libbluetooth by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

To contribute to libbluetooth, please see [CONTRIBUTING](CONTRIBUTING.md).
