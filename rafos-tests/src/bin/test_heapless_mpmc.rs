use std::sync::mpsc;
use std::thread;

use heapless::mpmc::MpMcQueue;

const N: u32 = 2;

static Q: MpMcQueue<u32, 2> = MpMcQueue::new();

fn main() {


    let (s, r) = mpsc::channel();
    thread::scope(|scope| {
        let s1 = s.clone();
        scope.spawn(move || {
            let mut sum: u32 = 0;

            for i in 0..(1 * N) {
                sum = sum.wrapping_add(i);
                println!("enqueue {}", i);
                while Q.enqueue(i).is_err() {}
            }

            s1.send(sum).unwrap();
        });

        let s2 = s.clone();
        scope.spawn(move || {
            let mut sum: u32 = 0;

            for _ in 0..(1 * N) {
                loop {
                    if let Some(v) = Q.dequeue() {
                        sum = sum.wrapping_add(v);
                        println!("dequeue {}", v);
                        break;
                    }
                }
            }

            s2.send(sum).unwrap();
        });
    });

    assert_eq!(r.recv().unwrap(), r.recv().unwrap());
}