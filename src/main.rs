use std::{thread, time::Duration};

fn main() {
    let mut n = 0u64;
    loop {
        n += 1;
        println!("hello from rust, tick {n}");
        thread::sleep(Duration::from_secs(60));
    }
}
