pub struct Stack<T> {
    inner: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        let inner = Vec::with_capacity(8);
        Self { inner }
    }

    pub fn push(&mut self, item: T) {
        self.inner.push(item)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(5);

        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
}
