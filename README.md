# cli-play
<!-- [![Crates.io](https://img.shields.io/crates/v/cli-play.svg)](https://crates.io/crates/cli-play) -->
<!-- [![Docs.rs](https://docs.rs/cli-play/badge.svg)](https://docs.rs/cli-play) -->
[![CI](https://github.com//cli-play/workflows/CI/badge.svg)](https://github.com//cli-play/actions)

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install cli-play`

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## trycmd test

expected usage:
```console
$ tk
hello thor

```

```console
$ simple World
Hello World!

$ simple Ferris
Hello Ferris!

```

```trycmd
$ ftzz ./exact -en 1M
Exactly 1,000,000 files will be generated in approximately 1,000 directories distributed across a tree of maximum depth 5 where each directory contains approximately 4 other directories.
Created 1,000,000 files across 1,259 directories.

```