# riff--

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/riff--)
[![Crates.io](https://img.shields.io/crates/v/riff--)](https://crates.io/crates/riff--)
[![Docs.rs](https://docs.rs/riff--/badge.svg)](https://docs.rs/riff--)

![Rust 1.51](https://img.shields.io/static/v1?logo=Rust&label=&message=1.51&color=grey)
[![CI](https://github.com/Tamschi/riff--/workflows/CI/badge.svg?branch=develop)](https://github.com/Tamschi/riff--/actions?query=workflow%3ACI+branch%3Adevelop)
![Crates.io - License](https://img.shields.io/crates/l/riff--/0.0.1)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/riff--)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/riff--)](https://github.com/Tamschi/riff--/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/riff--)](https://github.com/Tamschi/riff--/pulls)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/riff--.svg)](https://web.crev.dev/rust-reviews/crate/riff--/)

Extracts RIFF files from an unstructured binary. Bad code.

## Installation

```cmd
cargo install riff-- --git https://github.com/Tamschi/riff--.git --branch develop
```

## Usage

```cmd
REM Extract all embedded RIFF files into `<path>.0x<offset>.wav` files.
riff-- <path>
```

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`riff--` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

* The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
* The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).
