pub fn run() {
    //arrays are primitive
    let list1: [i32; 6] = [1,2,4,5,5,6];
    let list2 = list1;

    println!("lists {:?} {:?}",list1, list2 );

    let vec1: Vec<i32> = vec![1,2,3,4];
    let vec2 = &vec1;

    println!("{:?}", (&vec1, vec2));

}