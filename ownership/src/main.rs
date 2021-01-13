fn main() {
    // ex.1-1 動く。整数は既知の固定サイズの単純な値で、スタックに積まれるため
    let x = 5;
    let y = x;
    println!("x:{}, y:{}", x, y);

    // // ex.1-2 コンパイルエラー
    // // 中身のデータはヒープにあり、スタックにあるポインタ/長さ/許容量をコピーしている
    // // もし下のコードがエラーにならなければ、二重解放というメモリ安全性上のバグになりうる
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);

    // // ex.1-3 毎回move
    // let s1 = gives_ownership(); // gives_ownershipは、戻り値をs1にムーブする
    // let s2 = String::from("hello"); // s2がスコープに入る
    // let s3 = takes_and_gives_back(s2); // s2はtakes_and_gives_backにムーブされ、戻り値もs3にムーブされる

    // // ex.2-1 参照と借用
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // // '{}'の長さは、{}です
    // println!("The length of '{}' is {}.", s1, len);

    // // ex.2-2 可変な借用
    // let mut s = String::from("hello");
    // change(&mut s);
    // println!("String is {}", s)

    // // ex.2-3 コンパイルエラー。特定のスコープで、ある特定のデータに対しては、1つしか可変な参照を持てない
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2)

    // // ex.2-4 コンパイルエラー。不変な参照をしている間は、可変な参照をすることはできません
    // let mut s = String::from("hello");
    // let r1 = &s; // 問題なし
    // let r2 = &s; // 問題なし
    // let r3 = &mut s; // 大問題！
    // println!("{}, {}, {}", r1, r2, r3)

    // // ex.2-5 宙に浮いた参照
    // let reference_to_nothing = dangle();
}

// // gives_ownershipは、戻り値を呼び出した関数にムーブする
// fn gives_ownership() -> String {
//     let some_string = String::from("hello"); // some_stringがスコープに入る
//     some_string // some_stringが返され、呼び出し元関数にムーブされる
// }

// // takes_and_gives_backは、Stringを一つ受け取り、返す。
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_stringがスコープに入る。
//     a_string // a_stringが返され、呼び出し元関数にムーブされる
// }

// fn calculate_length(s: &String) -> usize { // sはStringへの参照
//     s.len()
// } // ここで、sはスコープ外になる。けど、参照しているものの所有権を持っているわけではないので、何も起こらない

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn dangle() -> &String { // dangleはStringへの参照を返す
//     let s = String::from("hello"); // sは新しいString
//     &s // String sへの参照を返す
// } // ここで、sはスコープを抜け、ドロップされる。そのメモリは消される。危険だ
