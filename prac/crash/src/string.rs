

pub fn run() {
    //primitive imutable
    let hello1 = "Hello";
    //head allocated
    let mut hello2 = String::from("Hello");

    hello2.push('W');
    hello2.push_str(" we");

    println!("{}", hello2.len());
    println!("Capacity {}", hello2.capacity());
    println!("{}", hello2.is_empty());
    println!("{}", hello2.contains("Hello"));
    println!("{}", hello2.replace("Hello", "World"));

    for letter in hello2.split_whitespace() {
        println!("{}", letter);
    }

    let mut s = String::with_capacity(10);
    for i in 0..10 {
        s.push(std::char::from_u32(i).unwrap());
    }

    println!("{:?}", s);
    assert_eq!(10, s.len());


}