use either::Either;

use crate::requests::Payload;

pub trait HasPayload {
    type Payload: Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload;

    fn payload_ref(&self) -> &Self::Payload;

    fn with_payload_mut<F>(mut self, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(&mut Self::Payload),
    {
        f(self.payload_mut());
        self
    }
}

impl<P> HasPayload for P
where
    P: Payload,
{
    type Payload = Self;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        self
    }

    fn payload_ref(&self) -> &Self::Payload {
        self
    }
}

impl<L, R> HasPayload for Either<L, R>
where
    L: HasPayload,
    R: HasPayload<Payload = L::Payload>,
{
    type Payload = L::Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        self.as_mut().either(<_>::payload_mut, <_>::payload_mut)
    }

    fn payload_ref(&self) -> &Self::Payload {
        self.as_ref().either(<_>::payload_ref, <_>::payload_ref)
    }
}
