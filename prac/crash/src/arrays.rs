pub fn run() {

    //arrays the length is fixed
    let mut numbers: [i32; 3] = [1,2,3];
    println!("{}", numbers[0]);
    numbers[0] = 5;
    println!("{:?}", numbers);
    println!("{}", numbers.len());
    //arrays are stack allocated
    println!("Szie in bytes {}", std::mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice : {:?}", slice);
}