use std::sync::Mutex;
use std::thread::scope;

fn main() {
    let counter = Mutex::new(0);
    scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                for _ in 0..1000000 {
                    let mut guard = counter.lock().unwrap();
                    *guard += 1;
                }
            });
        }
    });
    let total = counter.lock().unwrap();
    println!("Counter = {total}")
}
