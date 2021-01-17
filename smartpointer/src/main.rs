// use List::{Cons, Nil};
// use std::ops::Deref;

fn main() {
    // let b = Box::new(5);
    // println!("b = {}", b);

    // let list = Cons(1,
    //     Box::new(Cons(2,
    //         Box::new(Cons(3,
    //             Box::new(Nil))))));

    // let x = 5;
    // let y = &x;
    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // let x = 5;
    // let y = Box::new(x);
    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // let x = 5;
    // let y = MyBox::new(x);
    // assert_eq!(5, x);
    // assert_eq!(5, *y); // 水面下でコンパイラは、実際にはこのようなコードを走らせる *(y.deref())

    // let m = MyBox::new(String::from("Rust"));
    // hello(&m); // 参照外し型強制がなかったら hello(&(*m)[..]);

    // let c = CustomSmartPointer { data: String::from("my stuff") };
    // let d = CustomSmartPointer { data: String::from("other stuff") };
    // println!("CustomSmartPointers created.");

    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    // c.drop();
    std::mem::drop(c);
    // mainの終端の前にCustomSmartPointerがドロップされた
    println!("CustomSmartPointer dropped before the end of main.");
}

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

// // 参照外し型強制
// fn hello(name: &str) {
//     println!("Hello, {}!", name);
// }

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // CustomSmartPointerをデータ`{}`とともにドロップするよ
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
