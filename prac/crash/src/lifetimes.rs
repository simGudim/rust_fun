pub fn run() {
    let var1 = 10;
    let var2 = 430;

    let result = get_var(&var1, &var2);
    println!("{}", result);
}

fn get_var<'a, 'b: 'a>(param1: &'a i32, param2: &'b i32) -> &'a i32 {
    if param1 > param2 {
        param1
    } else {
        param2
    }
}
//need lifetime because more than one refrenced argument if only one argument than compliers knows which lifetime asign to the output
fn compare<'a, T: std::cmp::PartialOrd>(param1: &'a T, param2: &'a T) -> &'a T {
    if param1 > param2 {
        param1
    } else {
        param2
    }
}
fn compare1<T: std::cmp::PartialOrd>(param1: &T) -> &T {
    param1
}

struct Direction<'a, 'b> {
    list1: Vec<i32>,
    list2: &'a String,
    list3: &'b Vec<i32>
}