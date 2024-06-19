//! #### Simple and fast lockless async channels
//! Lightweight async channel that can be used to implement futures, streams,
//! notifiers, and actors.
//!
//! Whisk defines a simple [`Channel`] type rather than splitting into sender /
//! receiver pairs.  A [`Channel`] can both send and receive.
//!
//! # Optional Features
//!  - **futures_core_3**: Implement [`Stream`](futures_core_3::Stream) for
//!    [`Channel`] (generic `T` must be `Option<Item>`)
//!  - **event_iterator**: Implement
//!    [`EventIterator`](event_iterator::EventIterator) for [`Channel`]
//!
//! # Getting Started
//!
//! ```rust
//! use whisk::Channel;
//!
//! enum Cmd {
//!     /// Tell messenger to add
//!     Add(u32, u32, Channel<u32>),
//! }
//!
//! async fn worker_main(commands: Channel<Option<Cmd>>) {
//!     while let Some(command) = commands.recv().await {
//!         println!("Worker receiving command");
//!         match command {
//!             Cmd::Add(a, b, s) => s.send(a + b).await,
//!         }
//!     }
//!
//!     println!("Worker stopping…");
//! }
//!
//! async fn tasker_main() {
//!     // Create worker on new thread
//!     println!("Spawning worker…");
//!     let channel = Channel::new();
//!     let worker_task = worker_main(channel.clone());
//!     let worker_thread =
//!         std::thread::spawn(|| pasts::Executor::default().block_on(worker_task));
//!
//!     // Do an addition
//!     println!("Sending command…");
//!     let oneshot = Channel::new();
//!     channel.send(Some(Cmd::Add(43, 400, oneshot.clone()))).await;
//!     println!("Receiving response…");
//!     let response = oneshot.recv().await;
//!     assert_eq!(response, 443);
//!
//!     // Tell worker to stop
//!     println!("Stopping worker…");
//!     channel.send(None).await;
//!     println!("Waiting for worker to stop…");
//!
//!     worker_thread.join().unwrap();
//!     println!("Worker thread joined");
//! }
//!
//! # #[ntest::timeout(1000)]
//! fn main() {
//!     // Call into executor of your choice
//!     pasts::Executor::default().block_on(tasker_main());
//! }
//! ```

#![no_std]
#![doc(
    html_logo_url = "https://ardaku.github.io/mm/logo.svg",
    html_favicon_url = "https://ardaku.github.io/mm/icon.svg",
    html_root_url = "https://docs.rs/whisk"
)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]
#![deny(unsafe_code)]

extern crate alloc;

mod channel;
#[allow(unsafe_code)]
mod mutex;
mod queue;
#[allow(unsafe_code)]
mod wake_list;

pub use self::{channel::Channel, queue::Queue};
