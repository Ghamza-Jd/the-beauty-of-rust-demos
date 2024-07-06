fn main() {
    let mut v = vec![10; 4];
    let x = &v[0];
    println!("v[0] = {x}");

    for i in 0..1000 {
        v.push(i);
    }

    println!("v[0] = {x}");
}
