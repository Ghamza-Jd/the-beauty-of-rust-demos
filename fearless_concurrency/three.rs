fn main() {
    use std::sync::Arc;
    use std::thread;

    let data = Arc::new(vec![1, 2, 3]);
    let thread = thread::spawn({
        let data = data.clone();
        move || {
            println!("First thread: {:?}", data);
        }
    });
    println!("Main thread: {:?}", data);

    thread.join().unwrap();
}
