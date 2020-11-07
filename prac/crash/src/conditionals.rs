pub fn run() {
    let age: i32 = 18;
    let name = String::from("Simon");

    if age > 25 && !name.is_empty() || name.len() >2 {
        println!("Fuck you !");
    } else {
        println!("{:?}", ("ewiord", 2));
    }

    let x = if 5 > 6 { true } else { false };
}