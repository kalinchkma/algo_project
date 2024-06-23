
trait Med<T> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}

#[derive(Debug)]
struct Stack<T> {
    data: Vec<T>
}
impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack {
            data: Vec::new()
        }
    }
}

impl<T> Med<T> for Stack<T> {
    fn push(&mut self, item: T) {
        self.data.push(item);
    }
    fn size(&self) -> usize {
        self.data.len()
    }
    fn peek(&self) -> Option<&T> {
        self.data.last()
    }
    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

pub fn run() {
   let mut plate : Stack<i32> = Stack::new();
   println!("Check before push {}", plate.is_empty());

    plate.push(34);
    plate.push(90);
    plate.push(54);

    println!("Check after push {}", plate.is_empty());

    println!("My stack {:?} size {}", plate, plate.size());
    println!("Peek {:?}", plate.peek());
    
    println!("Pop {:?}", plate.pop());

    println!("After pop {:?} size {}", plate, plate.size());


}
