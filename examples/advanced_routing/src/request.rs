#[derive(Debug)]
pub enum RequestState<T> {
    Success(T),
    IsPending(bool),
}

impl<T> Default for RequestState<T> {
    fn default() -> Self {
        RequestState::IsPending(false)
    }
}
