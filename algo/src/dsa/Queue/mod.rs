#[derive(Debug)]
struct Queue<T> {
    items: Vec<T>
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue {
            items: Vec::new()
        }
    }

    fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    fn peek(&self) -> Option<&T> {
        Some(&self.items[0])
    }
    fn is_empty(self) -> bool {
        self.items.is_empty()
    }
    fn size(self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod queue_test {
    // import parent module access
    use super::*;

    #[test]
    fn test_queue_creation() {
        let q: Queue<i32> = Queue::new();
        let v: Vec<i32> = Vec::new();
        assert_eq!(q.items, v);
    }

    #[test]
    fn test_enqueu() {
        let mut q: Queue<i32> = Queue::new();
        let v: Vec<i32> = vec![32, 45, 90];

        q.enqueue(32);
        q.enqueue(45);
        q.enqueue(90);

        assert_eq!(q.items, v);
    }

    #[test]
    fn test_dequeue() {
        let mut q: Queue<i32> = Queue::new();

        q.enqueue(43);
        q.enqueue(55);

        assert_eq!(q.dequeue().unwrap(), 43);
        assert_eq!(q.dequeue().unwrap(), 55);
        assert_eq!(Some(q.dequeue()), None)
    }

}

pub fn run() {
    let mut q: Queue<i32> = Queue::new();

    q.enqueue(43);
    println!("First enq: {:?}", q);

    q.enqueue(55);
    println!("second enq: {:?}", q);

    q.enqueue(65);
    println!("3rd enq: {:?}", q);

    q.dequeue();
    println!("First dq: {:?}", q);

    q.dequeue();
    println!("Second dq: {:?}", q);

    q.dequeue();
    println!("3rd dq: {:?}", q);
}