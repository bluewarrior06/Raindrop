/// Object can be sent a message and have an error returned.
pub trait Messagable<State, M, E> {
    fn message(&self, state: &State, message: M) -> Option<E>;
}
pub trait MutMessagable<State, M, E> {
    fn message_mut(&mut self, state: &mut State, message: M) -> Option<E>;
}

/// Object can be queried.
pub trait Queryable<Q, T, E> {
    fn query(&self, query: Q) -> Result<T, E>;
}

/// Object can be queried.
pub trait MutQueryable<Q, T, E> {
    fn query_mut(&mut self, query: Q) -> Result<T, E>;
}
