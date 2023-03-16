fn main() {
    let mut my_bool = true;
    if my_bool  {
        println!("Hello!");
    } else {
        println!("Goodbuye!");
    }

    my_bool = false;

    if !my_bool {
        println!("It is false!");
    }  else {
        println!("It is true");
    }
}