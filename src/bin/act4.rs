fn main() {

    let some_bool = 2;

    match some_bool {
        15 => println!("it is True!!"),
        3 => println!("it is False!!"),
        1 => println!("It is 1"),
        2 => println!("It is 2"),
        _ => {println!("It is something other!")} //it is mandatory finishing of the match, means 'else'
    }
}