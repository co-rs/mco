/// stream is Iterator
pub use Iterator as Stream;

pub trait TryStream: Stream {
    /// The type of successful values yielded by this future
    type Ok;

    /// The type of failures yielded by this future
    type Error;

    /// Poll this `TryStream` as if it were a `Stream`.
    fn try_next(&mut self) -> Option<Result<Self::Ok, Self::Error>>;
}


impl<S, T, E> TryStream for S
    where
        S: ?Sized + Stream<Item=Result<T, E>>,
{
    type Ok = T;
    type Error = E;

    fn try_next(&mut self) -> Option<Result<Self::Ok, Self::Error>> {
        self.next()
    }
}