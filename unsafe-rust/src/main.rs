// 4つの行動をunsafe Rustでは行える
// - 生ポインタを参照外しすること
// - unsafeな関数やメソッドを呼ぶこと
// - 可変で静的な変数にアクセスしたり変更すること
// - unsafeなトレイトを実装すること

// use std::slice;

// extern "C" {
//     fn abs(input: i32) -> i32;
// }

// static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    // 参照やスマートポインタと異なり、生ポインタは:
    // - 同じ場所への不変と可変なポインタや複数の可変なポインタが存在することで借用規則を無視できる
    // - 有効なメモリを指しているとは保証されない
    // - nullの可能性がある
    // - 自動的な片付けは実装されていない

    // let mut num = 5;
    // // safeコードで生ポインタを生成できます。unsafeブロックの外では、生ポインタを参照外しできないだけなのです。
    // let r1 = &num as *const i32;
    // let r2 = &mut num as *mut i32;

    // let address = 0x012345usize;
    // let r = address as *const i32;

    // unsafe {
    //     println!("r1 is: {}", *r1);
    //     println!("r2 is: {}", *r2);
    // }

    // unsafe {
    //     dangerous();
    // }

    // unsafe {
    //     // -3の絶対値は、Cによると{}
    //     println!("Absolute value of -3 according to C: {}", abs(-3));
    // }

    // println!("name is: {}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// unsafe fn dangerous() {}

// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//     let ptr = slice.as_mut_ptr();

//     assert!(mid <= len);

//     unsafe {
//         (slice::from_raw_parts_mut(ptr, mid),
//          slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
//     }
// }

unsafe trait Foo {
    // methods go here
    // メソッドがここに来る
}
unsafe impl Foo for i32 {
    // method implementations go here
    // メソッドの実装がここに来る
}
