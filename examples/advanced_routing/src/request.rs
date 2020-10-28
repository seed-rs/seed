#[derive(Debug)]
pub enum RequestState<T> {
    Success(T),
    Failed { message: String, code: String },
    IsPending(bool),
}

impl<T> Default for RequestState<T> {
    fn default() -> Self {
        RequestState::IsPending(false)
    }
}
