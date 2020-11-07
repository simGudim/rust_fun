use std::collections::HashSet;

fn find_balloon<'a>(s: &'a mut String, chars: &'a HashSet<char>) -> i32 {
    let filtered: String = s.chars().filter(|x| chars.contains(x)).collect();
    count_chars(filtered)
}

fn count_chars(s: String) -> i32 {
    let mut count: Vec<i32> =vec![];
    count.push(s.matches("B").count() as i32);
    count.push(s.matches("A").count() as i32);
    let count_l = s.matches("L").count() as i32;
    if count_l % 2 == 0 && count_l > 1 {
        count.push(count_l / 2);
        count.push(count_l / 2);
    } else if (count_l - count_l % 2) % 2 == 0 && count_l > 1{
        count.push((count_l - count_l % 2) /2);
        count.push((count_l - count_l % 2) /2);
    } else if count_l == 1 {
        count.push(1);
        count.push(0);
    } else {
        count.push(0);
        count.push(0);
    }

    let count_0 = s.matches("O").count() as i32;
    if count_0 % 2 == 0 && count_l > 1 {
        count.push(count_0 / 2);
        count.push(count_0 / 2);
    } else if (count_0 - count_0 % 2) % 2 == 0 && count_0 > 1{
        count.push((count_0 - count_0 % 2) /2);
        count.push((count_0 - count_0 % 2) /2);
    } else if count_0 == 1 {
        count.push(1);
        count.push(0);
    } else {
        count.push(0);
        count.push(0);
    }

    count.push(s.matches("N").count() as i32);

    let minimum_value = match count.iter().min() {
        Some(min) => &min,
        None => &0
    };

    *minimum_value
}



fn main() {
    let mut var1 = String::from("BASDFSADLfasLOONBASDFSADLfasLOON");
    let chars: [char; 5] = ['B', 'A', 'L', 'O', 'N'];
    println!("{}", find_balloon(&mut var1, &chars.iter().cloned().collect()));
}