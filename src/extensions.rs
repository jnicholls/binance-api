use std::task::Poll;

#[easy_ext::ext(ResultExt)]
impl<T, E> Result<Result<T, E>, E> {
    pub fn x_flatten(self) -> Result<T, E> {
        self.and_then(std::convert::identity)
    }
}

#[easy_ext::ext(PollExt)]
impl<T, E> Poll<Option<Result<T, E>>> {
    pub fn x_map_ok<U, F>(self, f: F) -> Poll<Option<Result<U, E>>>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Poll::Ready(Some(Ok(t))) => Poll::Ready(Some(Ok(f(t)))),
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }

    pub fn x_map_err<U, F>(self, f: F) -> Poll<Option<Result<T, U>>>
    where
        F: FnOnce(E) -> U,
    {
        match self {
            Poll::Ready(Some(Ok(t))) => Poll::Ready(Some(Ok(t))),
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(f(e)))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }

    pub fn x_map_flatten<U, F>(self, f: F) -> Poll<Option<Result<U, E>>>
    where
        F: FnOnce(T) -> Result<U, E>,
    {
        self.map(|o| o.map(|r| r.map(f).x_flatten()))
    }
}
