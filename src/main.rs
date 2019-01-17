#![feature(futures_api)]

use std::future::Future;

use futures::future::{FutureExt, Shared};

#[derive(Clone)]
pub struct FooFuture<'a> {
    inner: Shared<Box<dyn Future<Output = Result<(), ()>> + Send + Unpin + 'a>>,
}

impl<'a> FooFuture<'a> {
    pub fn new<F>(future: F) -> Self
    where
        F: Future<Output = Result<(), ()>> + Send + 'a,
    {
        let inner: Box<dyn Future<Output = Result<(), ()>> + Send> = Box::new(future);
        FooFuture {
            inner: inner.shared(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
