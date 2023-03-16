use std::result;

enum Directions {
    Up,
    Down,
    Left,
    Right
}

fn which_way<'life>(go: Directions) -> &'life str {
    match go {
        Directions::Up => "up",
        Directions::Down => "down",
        Directions::Left => "left",
        Directions::Right => "right",
    }
}


fn main() {

    let result = which_way(Directions::Left);
    println!("{result:?}");


}