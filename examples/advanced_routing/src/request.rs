#[derive(Debug)]
pub enum State<T> {
    Success(T),
    IsPending(bool),
}

impl<T> Default for State<T> {
    fn default() -> Self {
        State::IsPending(false)
    }
}
