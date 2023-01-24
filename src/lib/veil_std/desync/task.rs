use core::{pin::Pin, future::Future, task::{Context, Poll}};

use alloc::boxed::Box;


pub struct Task {
    #[cfg(feature = "desync_atomics")]
    pub(crate) id: super::TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            #[cfg(feature = "desync_atomics")]
            id: super::TaskId::new(),
            future: Box::pin(future),
        }
    }
    pub(crate) fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}
unsafe impl Sync for Task {}
unsafe impl Send for Task {}