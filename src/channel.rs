use alloc::sync::Arc;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{wake_list::WakeHandle, Queue};

/// An MPMC channel with both send and receive capabilities
///
/// Enable the **`futures_core_3`** feature for `Channel` to implement
/// [`Stream`](futures_core_3::Stream) (generic `T` must be `Option<Item>`).
///
/// Enable the **`event_iterator`** feature for `Channel` to implement
/// [`EventIterator`](event_iterator::EventIterator).
pub struct Channel<T = (), U: ?Sized = ()>(Arc<Queue<T, U>>, WakeHandle);

impl<T, U: ?Sized> Drop for Channel<T, U> {
    fn drop(&mut self) {
        // Drop to avoid use after free
        self.1 = WakeHandle::new();
    }
}

impl<T> Channel<T> {
    /// Create a new channel.
    #[inline(always)]
    pub fn new() -> Self {
        Self(Arc::new(Queue::new()), WakeHandle::new())
    }
}

impl<T, U> Channel<T, U> {
    /// Create a new channel with associated data.
    #[inline(always)]
    pub fn with(user_data: U) -> Self {
        Self::from(Arc::new(Queue::with(user_data)))
    }
}

impl<T, U: ?Sized> Channel<T, U> {
    /// Send a message on this channel.
    #[inline(always)]
    pub async fn send(&self, message: T) {
        self.0.send(message).await
    }

    /// Receive a message from this channel.
    #[inline(always)]
    pub async fn recv(&self) -> T {
        self.0.recv().await
    }
}

impl<T, U: ?Sized> Clone for Channel<T, U> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0), WakeHandle::new())
    }
}

impl<T, U: ?Sized> core::fmt::Debug for Channel<T, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Channel").finish_non_exhaustive()
    }
}

impl<T, U: ?Sized + Default> Default for Channel<T, U> {
    fn default() -> Self {
        Self::with(U::default())
    }
}

impl<T, U: ?Sized> core::ops::Deref for Channel<T, U> {
    type Target = U;

    fn deref(&self) -> &Self::Target {
        &self.0.user
    }
}

impl<T, U: ?Sized> Future for Channel<T, U> {
    type Output = T;

    #[inline(always)]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let this = self.get_mut();
        this.0.data.take(cx, &mut this.1)
    }
}

#[cfg(feature = "event_iterator")]
impl<T, U: ?Sized> event_iterator::EventIterator for Channel<T, U> {
    type Event<'me> = T where Self: 'me;

    #[inline(always)]
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<T>> {
        let this = self.get_mut();

        this.0.data.take(cx, &mut this.1).map(Some)
    }
}

#[cfg(feature = "futures_core_3")]
impl<T, U: ?Sized> futures_core_3::Stream for Channel<Option<T>, U> {
    type Item = T;

    #[inline(always)]
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<T>> {
        let this = self.get_mut();
        this.0.data.take(cx, &mut this.1)
    }
}

impl<T, U: ?Sized> From<Arc<Queue<T, U>>> for Channel<T, U> {
    fn from(inner: Arc<Queue<T, U>>) -> Self {
        Self(inner, WakeHandle::new())
    }
}

impl<T, U: ?Sized> From<Channel<T, U>> for Arc<Queue<T, U>> {
    fn from(channel: Channel<T, U>) -> Self {
        channel.0.clone()
    }
}
