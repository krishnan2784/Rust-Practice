pub struct Groups<T> {
    inner: Vec<T>,
}

impl<T> Groups<T> {
    pub(crate) fn new(inner: Vec<T>) -> Self {
        Groups { inner }
    }
}

impl<T: PartialEq> Iterator for Groups<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // if the inner vector is empty, we are done
        if self.inner.is_empty() {
            return None;
        }

        // lets check the span of equal items
        let mut cursor = 1;
        let first = &self.inner[0];
        for element in &self.inner[1..] {
            if element == first {
                cursor += 1;
            } else {
                break;
            }
        }

        // we use the `Vec::drain` to extract items up until the cursor
        let items = self.inner.drain(0..cursor).collect();

        // return the extracted items
        Some(items)
    }

    // TODO: Write the rest of this implementation.
}