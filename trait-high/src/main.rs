// use std::ops::Add;

use std::fmt;

fn main() {
    // assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
    //     Point { x: 3, y: 3 });

    // let person = Human;
    // Pilot::fly(&person);
    // Wizard::fly(&person);
    // person.fly();

    // println!("A baby dog is called a {}", Dog::baby_name());
    // // println!("A baby dog is called a {}", Animal::baby_name());
    // println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // let p = Point { x: 1, y: 0 };
    // p.outline_print()

    // let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    // println!("w = {}", w);

    // type Kilometers = i32;
    // let x: i32 = 5;
    // let y: Kilometers = 5;
    // println!("x + y = {}", x + y);

    // // 長い型を多くの場所で使用する
    // let f: Box<Fn() + Send + 'static> = Box::new(|| println!("hi"));
    // fn takes_long_type(f: Box<Fn() + Send + 'static>) {
    //     // --snip--
    // }
    // fn returns_long_type() -> Box<Fn() + Send + 'static> {
    //     // --snip--
    // }
    // // 型エイリアス
    // type Thunk = Box<Fn() + Send + 'static>;
    // let f: Thunk = Box::new(|| println!("hi"));
    // fn takes_long_type(f: Thunk) {
    //     // --snip--
    // }
    // fn returns_long_type() -> Thunk {
    //     // --snip--
    // }
    // // std::ioにはこんな類のエイリアス宣言があります
    // type Result<T> = Result<T, std::io::Error>;

    // let answer = do_twice(add_one, 5);
    // // 答えは{}
    // println!("The answer is: {}", answer);

    // let list_of_numbers = vec![1, 2, 3];
    // // インラインでクロージャが定義される
    // let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    // // 名前付きの関数を使用
    // let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
}

// 関連型。1つのメソッドと関連型が1つあるトレイト
// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }
// impl Iterator for Counter {
//     type Item = u32;
//     fn next(&mut self) -> Option<Self::Item> {
//         // --snip--
//     }
// }

// // Counterに対してnextメソッドを使用する際に、どのIteratorの実装を使用したいか型注釈をつけなければならない
// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }

// #[derive(Debug, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }
// impl Add for Point {
//     type Output = Point;
//     fn add(self, other: Point) -> Point {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

// struct Millimeters(u32);
// struct Meters(u32);
// impl Add<Meters> for Millimeters {
//     type Output = Millimeters;
//     fn add(self, other: Meters) -> Millimeters {
//         Millimeters(self.0 + (other.0 * 1000))
//     }
// }

// trait Pilot {
//     fn fly(&self);
// }
// trait Wizard {
//     fn fly(&self);
// }
// struct Human;
// impl Pilot for Human {
//     fn fly(&self) {
//         // キャプテンのお言葉
//         println!("This is your captain speaking.");
//     }
// }
// impl Wizard for Human {
//     fn fly(&self) {
//         // 上がれ！
//         println!("Up!");
//     }
// }
// impl Human {
//     fn fly(&self) {
//         // *激しく腕を振る*
//         println!("*waving arms furiously*");
//     }
// }

// trait Animal {
//     fn baby_name() -> String;
// }
// struct Dog;
// impl Dog {
//     fn baby_name() -> String {
//         // スポット(Wikipediaによると、飼い主の事故死後もその人の帰りを待つ忠犬の名前の模様)
//         String::from("Spot")
//     }
// }
// impl Animal for Dog {
//     fn baby_name() -> String {
//         // 子犬
//         String::from("puppy")
//     }
// }

// trait OutlinePrint: fmt::Display {
//     fn outline_print(&self) {
//         let output = self.to_string();
//         let len = output.len();
//         println!("{}", "*".repeat(len + 4));
//         println!("*{}*", " ".repeat(len + 2));
//         println!("* {} *", output);
//         println!("*{}*", " ".repeat(len + 2));
//         println!("{}", "*".repeat(len + 4));
//     }
// }
// struct Point {
//     x: i32,
//     y: i32,
// }
// impl OutlinePrint for Point {}
// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

// // ニュータイプパターンを使用して外部の型に外部のトレイトを実装する
// // Vec<T>にDisplayを実装したいとしましょう。DisplayトレイトもVec<T>型もクレートの外で定義されているので、 直接それを行うことはオーファンルールにより妨げられます。Vec<T>のインスタンスを保持するWrapper構造体を作成できます; そして、WrapperにDisplayを実装し、Vec<T>値を使用できます。
// struct Wrapper(Vec<String>);
// impl fmt::Display for Wrapper {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[{}]", self.0.join(", "))
//     }
// }

// // never型は絶対に返らない
// fn bar() -> ! {
//     // --snip--
// }

// // 関数ポインタ
// fn add_one(x: i32) -> i32 {
//     x + 1
// }
// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }

// クロージャを返却する
// // コンパイルエラー
// fn returns_closure() -> Fn(i32) -> i32 {
//     |x| x + 1
// }
// // トレイトオブジェクトを使えます
// fn returns_closure() -> Box<Fn(i32) -> i32> {
//     Box::new(|x| x + 1)
// }
