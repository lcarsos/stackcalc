use std::option::Option;

pub struct Stack<T: Copy> {
    vec: Vec<T>,
}

impl<T: Copy> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            vec: Vec::new()
        }
    }

    pub fn push(&mut self, x: T) {
        self.vec.push(x);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.vec.last()
    }
}
