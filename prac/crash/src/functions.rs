pub fn run() {
    greetings("Hello", "Simon");
    let sum_get = adding(2, 3);

    //Closure can use outsiode varibales EX:
    let n3: i32 = 8;
    let add_nums = |n1: i32, n2: i32| n1 + n2 +n3;
    println!("{}", add_nums(3, 5));
}

fn greetings(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn adding(num1: i32, num2: i32) -> i32{
    num1 + num2
}

