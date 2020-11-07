pub fn run() {
    let mut count = 0;

    'a: loop {
        if count < 20 {
            count += 1
        } else {
            break 'a;
        }
    }

    while count < 40 {
        count += 1;
    }
    //,,= inclusive || .. exclusive
    for x in 0..=100{
        println!("{}", x);
    }
}