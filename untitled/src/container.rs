pub struct Container<T> {
    pub(crate) value: T,
}

impl<T> Container<T> {
    pub fn new(value: T) -> Self {
        Container { value }
    }
}