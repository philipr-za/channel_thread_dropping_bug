use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

struct Foo {
    data: i32,
    start: Instant,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!(
            "Dropping Foo with data {} - {:?}",
            self.data,
            self.start.elapsed()
        );
    }
}

fn main() {
    let start = Instant::now();
    let (tx1, rx1) = mpsc::sync_channel(1);

    let _ = thread::spawn(move || {
        // Move the receiver into this thread.
        let _rx = rx1;
        thread::sleep(Duration::from_secs(1));
        panic!("expected panic");
        // Under 1.66.1 sender `tx1` and the item in it's buffer are dropped after this panic causes
        // the thread to unwind and the Receiver to be dropped
        // Under 1.67.0 the sender `tx1` and the data in it's buffer are not dropped here
    });

    tx1.send(Foo { data: 1, start }).unwrap();
    println!("Sends completed - {:?}", start.elapsed());

    thread::sleep(Duration::from_secs(5));
    println!("Ending - {:?}", start.elapsed());
    // Under 1.67.0 the sender `tx1` and the data in it's buffer are only dropped when the
    // whole process ends here
}
