# shapely-derive

[![experimental](https://img.shields.io/badge/status-highly%20experimental-orange)](https://github.com/fasterthanlime/shapely)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/fasterthanlime/free-of-syn)
[![crates.io](https://img.shields.io/crates/v/shapely-derive.svg)](https://crates.io/crates/shapely-derive)
[![documentation](https://docs.rs/shapely-derive/badge.svg)](https://docs.rs/shapely-derive)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/shapely-derive.svg)](./LICENSE)

shapely-derive provides procedural macros to derive the `Shapely` trait from shapely.

This crate implements the `#[derive(Shapely)]` macro which automatically generates runtime reflection code for Rust structs, providing:

  * Structure parsing and representation
  * Field access and manipulation
  * Integration with the shapely runtime reflection system

The implementation uses [unsynn](https://crates.io/crates/unsynn) for efficient and fast compilation.

### Funding

Thanks to Namespace for providing fast GitHub Actions workers:

<a href="https://namespace.so"><img src="./static/namespace-d.svg" height="40"></a>

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
