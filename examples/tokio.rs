use std::{
    num::NonZeroUsize,
    pin::Pin,
    task::{Context, Poll},
};

use futures_core_3::Stream;
use tokio::task;
use tokio_stream::StreamExt;
use whisk::Channel;

struct FilterZeroStream<T>(T);

impl<T> Stream for FilterZeroStream<T>
where
    T: Stream<Item = usize> + Unpin,
{
    type Item = usize;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        while let Poll::Ready(output) = Pin::new(&mut self.0).poll_next(cx) {
            let Some(output) = output else {
                return Poll::Ready(None);
            };

            let Some(output) = NonZeroUsize::new(output) else {
                continue;
            };

            return Poll::Ready(Some(output.into()));
        }

        Poll::Pending
    }
}

/// Expects a channel that sends one non-zero value before closing.
async fn worker(channel: Channel<Option<usize>>) -> usize {
    let mut filter = FilterZeroStream(channel);
    let result = filter.next().await.unwrap();

    assert!(filter.next().await.is_none());

    result
}

#[tokio::main]
async fn main() {
    let channel = Channel::new();
    let channel_clone = channel.clone();
    let join_handle = task::spawn(async move { worker(channel_clone).await });

    channel.send(Some(0)).await;
    channel.send(Some(12)).await;
    channel.send(None).await;

    let output = join_handle.await.unwrap();

    assert_eq!(output, 12);
}
