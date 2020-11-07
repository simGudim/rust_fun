pub fn run() {

    //arrays the length is fixed
    let mut numbers: Vec<i32> = vec![1,2,3];
    println!("{}", numbers[0]);
    numbers[0] = 5;
    println!("{:?}", numbers);
    println!("{}", numbers.len());
    //arrays are stack allocated
    println!("Szie in bytes {}", std::mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice : {:?}", slice);

    numbers.push(5);
    numbers.pop();
    println!("{:?}", numbers);

    // for x in numbers {
    //     println!("{}", x);
    // }

    for x in numbers.iter() {
        println!("{}", x);
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", numbers);
}