fn main() {
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;

    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let thread = thread::spawn({
        let data = data.clone();
        move || {
            let mut guard = data.lock().unwrap();
            guard.push(31);
            println!("First thread: {:?}", guard);
        }
    });

    let thread_two = thread::spawn({
        let data = data.clone();
        move || {
            let mut guard = data.lock().unwrap();
            guard.push(31);
            println!("Second thread: {:?}", guard);
        }
    });

    println!("Main thread: {:?}", data.lock().unwrap());
    drop(data);
    thread.join().unwrap();
    thread_two.join().unwrap();
}
