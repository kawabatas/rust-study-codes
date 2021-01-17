// 並行プログラミングは、プログラムの異なる部分が独立して実行すること
// 並列プログラミングは、プログラムの異なる部分が同時に実行すること

// マルチスレッドで起きる問題
// - スレッドがデータやリソースに矛盾した順番でアクセスする競合状態
// - 2つのスレッドがお互いにもう一方が持っているリソースを使用し終わるのを待ち、両者が継続するのを防ぐデッドロック
// - 特定の状況でのみ起き、確実な再現や修正が困難なバグ

use std::thread;
use std::time::Duration;
use std::sync::mpsc; // multiple producer, single consumer 複数の送信側と、たった1つの受信側
                     // 複数の小川が互いに合わさって1つの大きな川になるところを想像してください


// mutual exclusion(相互排他)
// どんな時も1つのスレッドにしかなんらかのデータへのアクセスを許可しない
// ミューテックスは、2つの規則を覚えておく必要があるため、難しいという評判があります:
// - データを使用する前にロックの獲得を試みなければならない。
// - ミューテックスが死守しているデータの使用が終わったら、他のスレッドがロックを獲得できるように、 データをアンロックしなければならない。
use std::sync::{Mutex, Arc}; // RefCell<T>/Rc<T>とMutex<T>/Arc<T>の類似性
// use std::rc::Rc;

fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // handle.join().unwrap();
    // for i in 1..5 {
    //     // メインスレッドから数字{}だよ！
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(move || {
    //     // こちらがベクタ: {:?}
    //     println!("Here's a vector: {:?}", v);
    // });
    // handle.join().unwrap();

    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap(); // send関数は引数の所有権を奪い、 値がムーブされると、受信側が所有権を得る
    //     // println!("val is {}", val); // 所有権がないのでエラー
    // });
    // let received = rx.recv().unwrap();
    // // 値は{}です
    // println!("Got: {}", received);

    // let (tx, rx) = mpsc::channel();
    // let tx1 = mpsc::Sender::clone(&tx);
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];
    //     for val in vals {
    //         tx1.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    // thread::spawn(move || {
    //     // 君のためにもっとメッセージを(more messages for you)
    //     let vals = vec![
    //         String::from("more"),
    //         String::from("messages"),
    //         String::from("for"),
    //         String::from("you"),
    //     ];
    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    // for received in rx {
    //     println!("Got: {}", received);
    // }

    // シングルスレッド
    // let m = Mutex::new(5);
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    // println!("m = {:?}", m);

    // マルチスレッドでMutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
