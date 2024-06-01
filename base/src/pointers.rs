#![allow(dead_code)]

fn box_pointer() {
    let num = Box::new(5);
    println!("num: {}", num);
    println!("*num: {}", num);

    #[derive(Debug, Clone)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    impl List {
        fn new() -> List {
            List::Nil
        }

        fn append(self, elem: i32) -> List {
            List::Cons(elem, Box::new(self))
        }

        fn pop(&mut self) -> Option<i32> {
            match *self {
                List::Cons(head, ref mut tail) => {
                    *self = *tail.clone();
                    Some(head)
                },
                List::Nil => {
                    None
                }
            }
        }

        fn len(&self) -> i32 {
            match *self {
                List::Cons(_, ref tail) => 1 + tail.len(),
                List::Nil => 0
            }
        }
        
     
        fn stringify(&self) -> String {
            match *self {
                List::Cons(head, ref tail) => {
                    format!("{}, {}", head, tail.stringify())
                },
                List::Nil => {
                    format!("Nil")
                }
            }
        }

    }

    // let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    // println!("{:?}", list);
    let mut numbers = List::new();

    numbers = numbers.append(1);
    numbers = numbers.append(2);
    numbers = numbers.append(3);
    numbers = numbers.append(4);
    numbers = numbers.append(5);
    numbers = numbers.append(6);
    numbers = numbers.append(7);
    numbers = numbers.append(8);
    numbers = numbers.append(9);

    println!("List: {:?}", numbers.stringify());
}


use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


fn deref_traits() {
    let x = 100;
    let y = &x;
    let y2 = Box::new(x);
    let y3 = MyBox::new(x);

    assert_eq!(100, x);
    assert_eq!(100, *y);
    // assert_eq!(100, y); // error
    println!("x: {}, y: {}", x, y);

    assert_eq!(100, *y2);
    assert_eq!(100, y3.0);
    assert_eq!(100, *y3);
}


fn drop_traits() {
    use std::ops::Deref;
    // creating custom smart pointer
    struct CustomSmartPointer {
        data: String
    }

    // implement drop trait
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`", self.data);
        }
    }

    // implement deref trait
    impl Deref for CustomSmartPointer {
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    // implement customsmartpointer
    impl CustomSmartPointer {
        fn new(data: String) -> CustomSmartPointer {
            CustomSmartPointer {
                data
            }
        }
    }

    // using custom smart pointer
    let a = CustomSmartPointer::new("My variable of A".to_string());
    let b = CustomSmartPointer::new("My variable of B".to_string());

    println!("Printing variable {}", *a)
}


fn reference_counting() {
    use std::rc::Rc;

    enum MyList<T> {
        Cons(T, Rc<MyList<T>>),
        Nill
    }

    // use list
    let a = Rc::new(MyList::Cons(4, Rc::new(MyList::Cons(5, Rc::new(MyList::Cons(6, Rc::new(MyList::Nill)))))));
    println!("count reference after creating a, {}", Rc::strong_count(&a));
    let b = MyList::Cons(7, Rc::clone(&a));
    println!("count reference after creating b, {}", Rc::strong_count(&a));
    {
        let c = MyList::Cons(8, Rc::clone(&a));
        println!("Reference after creating c, {}", Rc::strong_count(&a));
    }
    println!("Reference after c gose out of scope, {}", Rc::strong_count(&a))
}

fn interior_mutability() {
    
}

pub fn run() {
    // box_pointer();
    // deref_traits();
    // drop_traits()
    // reference_counting();
}