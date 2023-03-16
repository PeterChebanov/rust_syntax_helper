
fn main() {

    enum Access {
        Full,
    }

    fn one_two_there() -> (i32, i32, i32) {
        (1, 2, 3)
    }


    let numbers = one_two_three();
    let(x, y, z) = one_two_there();

    println!("{:?}, {:?}", x, numbers.0);
    println!("{:?}, {:?}", x, numbers[1]);
    println!("{:?}, {:?}", x, numbers.2);

    let(employee, access) = ("Jake", Access::Full);


}