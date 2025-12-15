#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }

    // 入栈：始终保证q1队首是栈顶（最新元素）
    pub fn push(&mut self, elem: T) {
        // 1. 新元素先入队到空的q2
        self.q2.enqueue(elem);
        // 2. 将q1所有元素移到q2，此时q2顺序：新元素 + 原q1元素
        while !self.q1.is_empty() {
            self.q2.enqueue(self.q1.dequeue().unwrap());
        }
        // 3. 交换q1/q2，q1成为存储栈元素的主队列
        std::mem::swap(&mut self.q1, &mut self.q2);
    }

    // 出栈：直接弹出q1队首（栈顶）
    pub fn pop(&mut self) -> Result<T, &str> {
        match self.q1.dequeue() {
            Ok(val) => Ok(val),
            Err(_) => Err("Stack is empty"),
        }
    }

    // 栈空判断：仅判断主队列q1
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}