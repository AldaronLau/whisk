# Whisk
[![tests](https://github.com/ardaku/whisk/actions/workflows/ci.yml/badge.svg)](https://github.com/ardaku/whisk/actions/workflows/ci.yml)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/y/ardaku/whisk)](https://github.com/ardaku/whisk/)
[![GitHub contributors](https://img.shields.io/github/contributors/ardaku/whisk)](https://github.com/ardaku/whisk/graphs/contributors)  
[![Crates.io](https://img.shields.io/crates/v/whisk)](https://crates.io/crates/whisk)
[![Crates.io](https://img.shields.io/crates/d/whisk)](https://crates.io/crates/whisk)
[![Crates.io (recent)](https://img.shields.io/crates/dr/whisk)](https://crates.io/crates/whisk)  
[![Crates.io](https://img.shields.io/crates/l/whisk)](https://github.com/ardaku/whisk/search?l=Text&q=license)
[![Docs.rs](https://docs.rs/whisk/badge.svg)](https://docs.rs/whisk/)

#### Simple and fast async channels
Simple and fast async channels that can be used to implement futures, streams,
notifiers, and actors.  Whisk is purposely kept small, implemented in under 500
lines of code - and also works on `no_std`!

## MSRV
Whisk targets Rust 1.59.0 and later.

## Benchmarks
Benchmarks for v0.3.0 mpmc call on pasts runtime (compared to dynamic library):

```
Dynamic library: 6ns
Whisk (2-thread): 5.835µs
Flume (2-thread): 6.051µs
Whisk (1-thread): 281ns
Flume (1-thread): 317ns
```
