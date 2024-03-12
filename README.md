# jumbf

A [JUMBF (ISO/IEC 19566-5:2019)] parser and builder written in pure Rust.

[![CI](https://github.com/scouten-adobe/jumbf-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/scouten-adobe/jumbf-rs/actions/workflows/ci.yml)  [![Latest Version](https://img.shields.io/crates/v/jumbf.svg)](https://crates.io/crates/jumbf) [![docs.rs](https://img.shields.io/docsrs/jumbf)](https://docs.rs/jumbf/latest/jumbf/) [![codecov](https://codecov.io/gh/scouten-adobe/jumbf-rs/graph/badge.svg?token=di7n9t9B80)](https://codecov.io/gh/scouten-adobe/jumbf-rs)

The parser is implemented with the [nom] parser combinator framework and makes extensive use of zero-copy.

The builder can be built by itself and has no third-party crate dependencies in that configuration.


This crate is intentionally minimal in its understanding of box content. Only `jumb` (superbox) and `jumd` (description box) content are understood. The content of all other box types is application-specific and thus the meaning of that content is left to the caller.

## Crate features

Since the parsing features of this crate include dependencies on [nom] and [thiserror], those features are gated on a crate feature named `parser`, which is included by default.

If you only need to _build_ JUMBF data structures and want to reduce compile-time overhead, you can disable the `parser` feature by importing this crate as follows:

```toml
jumbf = { version = "x.x", default-features = false }
```

## Contributions and feedback

We welcome contributions to this project. For information on contributing, providing feedback, and about ongoing work, see [Contributing](./CONTRIBUTING.md).

## Requirements

The toolkit requires **Rust version 1.74.0** or newer. When a newer version of Rust becomes required, a new minor (1.x.0) version of this crate will be released.

### Supported platforms

The toolkit has been tested on the following operating systems:

* Windows (IMPORTANT: Only the MSVC build chain is supported on Windows. We would welcome a PR to enable GNU build chain support on Windows.)
* MacOS (Intel and Apple silicon)
* Ubuntu Linux on x86 and ARM v8 (aarch64)

## License

The `jumbf` crate is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT).

Note that some components and dependent crates are licensed under different terms; please check the license terms for each crate and component for details.

## Changelog

Refer to the [CHANGELOG](https://github.com/contentauth/c2pa-rs/blob/main/CHANGELOG.md) for detailed changes derived from Git commit history.

[JUMBF (ISO/IEC 19566-5:2019)]: https://www.iso.org/standard/73604.html
[nom]: https://github.com/rust-bakery/nom
[thiserror]: https://crates.io/crates/thiserror
