fn main() {
    use std::rc::Rc;
    use std::sync::Mutex;
    use std::thread;

    let data = Rc::new(Mutex::new(vec![1, 2, 3]));
    let thread = thread::spawn({
        move || {
            let mut guard = data.lock().unwrap();
            guard.push(31);
            println!("First thread: {:?}", guard);
        }
    });
    println!("Main thread: {:?}", data.lock().unwrap());

    thread.join().unwrap();
}
