use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub struct DummyFuture;

impl Future for DummyFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

/// A future that returns `Pending` a bunch of times before returning `Ready`.
#[cfg(any(all(feature = "alloc", feature = "sync"), feature = "futures03"))]
pub struct SlowFuture {
    countdown: i32,
}

#[cfg(any(all(feature = "alloc", feature = "sync"), feature = "futures03"))]
impl SlowFuture {
    pub fn new() -> Self {
        Self { countdown: 10 }
    }
}

#[cfg(any(all(feature = "alloc", feature = "sync"), feature = "futures03"))]
impl Future for SlowFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        this.countdown -= 1;
        if this.countdown == 0 {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
