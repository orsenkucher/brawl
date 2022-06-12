use std::future::Future;

// use either::Either;

use crate::requests::{HasPayload, Output};

/// A ready-to-send Brawl request.
pub trait Request: HasPayload {
    /// The type of an error that may happen while sending a request to the API.
    type Err: std::error::Error + Send;

    /// The type of the future returned by the [`send`](Request::send) method.
    type Send: Future<Output = Result<Output<Self>, Self::Err>> + Send;

    /// The type of the future returned by the [`send_ref`](Request::send_ref) method.
    type SendRef: Future<Output = Result<Output<Self>, Self::Err>> + Send;

    /// Send this request.
    // And also, here we are creating custom future. Other possibility was to use async-trait crate.
    fn send(self) -> Self::Send;

    /// Send this request by reference.
    fn send_ref(&self) -> Self::SendRef;
}

// impl<L,R> Request for Either<L,R>
// where L:Request,
// R:Request<Payload = L::Payload, Err = L::Err>,
// {
//     type Err = L::Err;

//     type Send = future
// }
