pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Right;
    let avatar3 = Movement::Up;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);

}

enum Movement {
    Up,
    Down, 
    Left,
    Right
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("moving up"),
        Movement::Down => println!("moving down"),
        Movement::Left => println!("moving left"),
        Movement::Right => println!("moving right")
    }
}