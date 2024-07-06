fn main() {
    use std::thread;

    let data = vec![1, 2, 3];
    let thread = thread::spawn(move || {
        println!("First thread: {:?}", data);
    });
    println!("Main thread: {:?}", data);

    thread.join().unwrap();
}
