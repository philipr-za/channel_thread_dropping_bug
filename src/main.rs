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
    let (tx2, rx2) = mpsc::sync_channel(0);

    let _ = thread::spawn(move || {
        let _ = rx1;
        thread::sleep(Duration::from_secs(1));
        panic!("expected panic");
        // Under Stable 1.66.1 sender `tx1` and the item in it's buffer are dropped after this panic
        // Under Beta 1.67.0 the sender `tx1` and the data in it's buffer are not dropped here
    });
    let _ = thread::spawn(move || rx2.recv());

    tx1.send(Foo { data: 1, start }).unwrap();
    tx2.send(Foo { data: 2, start }).unwrap();

    println!("Sends completed - {:?}", start.elapsed());

    thread::sleep(Duration::from_secs(5));
    println!("Ending - {:?}", start.elapsed());
    // Under Beta 1.67.0 the sender `tx1` and the data in it's buffer are only dropped when the
    // whole process ends here
}
