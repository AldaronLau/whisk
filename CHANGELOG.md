# Changelog
All notable changes to `whisk` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://github.com/AldaronLau/semver).

## [0.13.0] - 2024-06-19
### Changed
 - Switched out `pasts` for `event_iterator`
 - Renamed `futures-core` feature to `futures_core_3`
 - Bumped MSRV to 1.70

## [0.12.1] - 2023-06-02
### Fixed
 - Weirdness with `Cargo.toml`'s `[[example]]` section and no-std example in CI.

## [0.12.0] - 2023-06-02
### Fixed
 - Default features being pulled in for `pasts` and `futures-core` optional
   dependencies when enabled

## [0.11.0] - 2023-02-19
### Changed
 - Update pasts to 0.14.x

## [0.10.0] - 2023-01-18
### Changed
 - Update pasts to 0.13.x

## [0.9.1] - 2022-11-26
### Fixed
 - Use after free bug (drop order)

## [0.9.0] - 2022-11-25
### Added
 - Implement `From` converting `Arc<Queue>` to `Channel`
 - Implement `From` converting `Channel` to `Arc<Queue>`
 - Separate `Queue` and `Channel` types
 - `Channel::with()` for storing additional data within the internal `Arc`
 - `Queue::with()` for storing additional data within
 - `Deref` implementation for accessing additional use data within `Channel`
 - `Deref` implementation for accessing additional use data within `Queue`

### Changed
 - Changed to lockless implementation (no more spinlocks)
 - Old `Channel` type is now called `Queue`
 - `Future`, `Notifier` and `Stream` are now implemented directly on `Channel`
   rather than on a shared reference.
 - Bump MSRV to Rust 1.65

### Removed
 - `Chan`, `Stream`, `WeakChan` and `WeakStream` type aliases

## [0.8.0] - 2022-09-28
### Added
 - `Chan`, `Stream`, `WeakChan`, `WeakStream` type aliases

### Removed
 - `Weak`, no longer necessary

### Changed
 - MSRV bumped to 1.64.0
 - `Channel::new()` no longer contains an `Arc`, so it's no longer `Clone`.  The
   usage of `Arc` is now up to the user of the library.
 - `Channel::new()` is now a `const fn`

## [0.7.0] - 2022-08-19
### Changed
 - `Weak<T>` and `Channel<T>` now have a default for generics: `T = ()`

## [0.6.0] - 2022-08-14
### Added
 - `Weak::try_send()`
 - `Weak::try_recv()`

### Changed
 - `Channel::send()` back to `async fn`
 - `Channel::recv()` back to `async fn`

## [0.5.0] - 2022-08-06
### Added
 - `Weak` reference to a `Channel`
 - Implement `Future` for `&Channel`
 - Implement `Notifier` for `&Channel`
 - Implement `Stream` for `&Channel`

### Changed
 - Updated pasts to 0.12
 - `Channel::recv()` from an `async fn` to a `fn() -> impl Future`
 - `Channel::send()` from returning `Message` to returning `impl Future`
 - `Channel::recv()` no longer requires a mutable reference
 - `Channel<T>` now supports `T: !Unpin`

### Removed
 - `Message`
 - Const generic arguments on `Channel`
 - Panic on uncompleted message send

### Fixed
 - Bug with wakers when using MPMC functionality that could possibly trigger UB

## [0.4.1] - 2022-07-23
### Fixed
 - Error in documentation

## [0.4.0] - 2022-07-22
### Added
 - **`pasts`** feature for `Channel<T>` to implement `Notifier`
 - **`futures-core`** feature for `Channel<Option<T>>` to implement `Stream`
 - `Message` future type
 - `Channel::new()` for creating a channel
 - `Channel::send()` for sending a message on a channel
 - `Channel::recv()` for receiving a message from a channel

### Changed
 - Channels are now MPMC instead of SPSC
 - Less unsafe code, simpler implementation
 - Channels are now symmetrical

### Removed
 - `Sender` - Functionality built into `Channel` now
 - `Receiver` - Functionality built into `Message` now
 - `Worker` - Functionality built into `Message` now
 - `Tasker` - Functionality built into `Channel` now
 - **`std`** feature
 - `Channel::to_pair()` - Use `Channel::new()` instead
 - `Channel::pair()` - Use `Channel::new()` instead

## [0.3.0] - 2022-07-10
### Added
 - `Channel` struct, and `Channel::to_pair()` for reusing oneshot channels
 - `Sender` and `Receiver` oneshot-rendezvous channels
 - `Worker::stop()` to close a spsc channel

### Changed
 - Replaced `channel()`, with `Channel::pair()`
 - Renamed `Commander` to `Worker`, and `Messenger` to `Tasker`

### Removed
 - `Command`, `Message` types
 - `Commander::start()` and `Messenger::start()`, use `Worker::send()` and
   `Commander::recv_next()` instead

## [0.2.1] - 2022-04-19
### Changed
 - Small optimizations

## [0.2.0] - 2022-04-19
### Added
 - `start()` async methods on both `Commander` and `Messenger` for initiating
   the connection.

### Changed
 - `Commander` and `Messenger` changed from `Future`s to `Iterator`s.

### Removed
 - `close()` methods on both `Commander` and `Messenger`, now you can just rely
   on dropping to close channels.

### Fixed
 - Incorrect assumptions about wakers

## [0.1.1] - 2022-03-04
### Fixed
 - Deadlock condition caused by wrong drop order

## [0.1.0] - 2022-02-27
### Added
 - `Command`
 - `Commander`
 - `Message`
 - `Messenger`
 - `channel()`
