enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_player(m: Movement) {
    match m {
        Movement::Up => println!("moving up"),
        Movement::Down => println!("moving down"),
        Movement::Left => println!("moving left"),
        Movement::Right => println!("moving right"),
    }
}

pub fn run() {
    let player1 = Movement::Up;
    let player2 = Movement::Down;
    let player3 = Movement::Left;
    let player4 = Movement::Right;

    move_player(player1);
    move_player(player2);
    move_player(player3);
    move_player(player4);
}
