#![feature(futures_api)]

use std::future::Future;

use futures::future::{FutureExt, FutureObj, Shared};

/// This works.
#[derive(Clone)]
pub struct FooFuture<'a> {
    inner: Shared<FutureObj<'a, Result<(), ()>>>,
}

impl<'a> FooFuture<'a> {
    pub fn new<F>(future: F) -> Self
    where
        F: Future<Output = Result<(), ()>> + Send + 'a,
    {
        let inner = FutureObj::new(Box::new(future));
        FooFuture {
            inner: inner.shared(),
        }
    }
}

/// This does not work.
#[derive(Clone)]
pub struct BarFuture<'a> {
    inner: Shared<Box<dyn Future<Output = Result<(), ()>> + Send + Unpin + 'a>>,
}

impl<'a> BarFuture<'a> {
    pub fn new<F>(future: F) -> Self
    where
        F: Future<Output = Result<(), ()>> + Send + 'a,
    {
        let inner: Box<dyn Future<Output = Result<(), ()>> + Send> = Box::new(future);
        BarFuture {
            inner: inner.shared(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
