//! Brawl Stars API requests.

pub use self::{
    has_payload::HasPayload, json::JsonRequest, payload::Payload, request::Request,
    requester::Requester,
};

mod has_payload;
mod json;
mod payload;
mod request;
mod requester;

/// A type that is returned after making a request to Brawl.
pub type ResponseResult<T> = Result<T, crate::RequestError>;

/// An output type of [`Payload`] in [`HasPayload`].
pub type Output<T> = <<T as HasPayload>::Payload as Payload>::Output;
