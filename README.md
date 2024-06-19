# Whisk

[![tests](https://github.com/ardaku/whisk/actions/workflows/ci.yml/badge.svg)](https://github.com/ardaku/whisk/actions/workflows/ci.yml)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/y/ardaku/whisk)](https://github.com/ardaku/whisk/)
[![GitHub contributors](https://img.shields.io/github/contributors/ardaku/whisk)](https://github.com/ardaku/whisk/graphs/contributors)  
[![Crates.io](https://img.shields.io/crates/v/whisk)](https://crates.io/crates/whisk)
[![Crates.io](https://img.shields.io/crates/d/whisk)](https://crates.io/crates/whisk)
[![Crates.io (recent)](https://img.shields.io/crates/dr/whisk)](https://crates.io/crates/whisk)  
[![Crates.io](https://img.shields.io/crates/l/whisk)](https://github.com/ardaku/whisk/search?l=Text&q=license)
[![Docs.rs](https://docs.rs/whisk/badge.svg)](https://docs.rs/whisk/)

#### Simple and fast lockless async channels

Simple and fast async channels that can be used to implement futures, streams,
notifiers, and actors.  Whisk is purposely kept small, implemented in under 1000
lines of Rust code, with zero dependencies (not including feature flags to
enable implementation of traits from other crates) - and also works on `no_std`!

## Benchmarks

Naïve benchmarks for v0.10.0 actor on pasts runtime (compared with dynamic
library):

> ```
> Dynamic library: 6ns
> Whisk (2-thread): 4.396µs
> Flume (2-thread): 4.594µs
> Whisk (1-thread): 277ns
> Flume (1-thread): 325ns
> ```

## MSRV

The current MSRV is Rust 1.70.

MSRV is updated according to the [Ardaku MSRV guidelines].

## License

Copyright © 2022-2024 The Whisk Crate Contributor(s)

Licensed under any of
 - Apache License, Version 2.0, ([LICENSE_APACHE] or
   <https://www.apache.org/licenses/LICENSE-2.0>)
 - Boost Software License, Version 1.0, ([LICENSE_BOOST] or
   <https://www.boost.org/LICENSE_1_0.txt>)
 - MIT License, ([LICENSE_MIT] or <https://mit-license.org/>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Help

If you want help using or contributing to this library, feel free to send me an
email at <aldaronlau@gmail.com>.

[Ardaku MSRV guidelines]: https://github.com/ardaku/.github/blob/v1/profile/MSRV.md
[LICENSE_APACHE]: https://github.com/ardaku/whisk/blob/v0/LICENSE_APACHE
[LICENSE_MIT]: https://github.com/ardaku/whisk/blob/v0/LICENSE_MIT
[LICENSE_BOOST]: https://github.com/ardaku/whisk/blob/v0/LICENSE_BOOST
