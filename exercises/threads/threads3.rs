// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.



use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Arc<Queue>, tx: mpsc::Sender<u32>) {
    let tx_clone = tx.clone();
    let q_clone = Arc::clone(&q);

    let first_half_thread = thread::spawn(move || {
        for val in &q_clone.first_half {
            println!("sending {:?}", val);
            tx_clone.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let q_clone = Arc::clone(&q);
    let tx_clone = tx.clone();
    let second_half_thread = thread::spawn(move || {
        for val in &q_clone.second_half {
            println!("sending {:?}", val);
            tx_clone.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    first_half_thread.join().unwrap();
    second_half_thread.join().unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Queue::new());
    let queue_length = queue.length;

    send_tx(Arc::clone(&queue), tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
        if total_received >= queue_length {
            break;
        }
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
