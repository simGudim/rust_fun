pub fn run() {
    let tup1: (u8, &str, bool) = (4, "Hello", true);
    println!("{:?}, {} {}", tup1, tup1.0, tup1.1);
}