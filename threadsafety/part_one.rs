use std::thread::scope;

fn main() {
    let mut counter = 0;
    // scoped threads will auto join when the scope ends
    scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                for _ in 0..1000000 {
                    counter += 1;
                }
            });
        }
    });
    println!("Counter = {counter}")
}
