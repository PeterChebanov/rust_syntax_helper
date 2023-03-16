struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

fn main() {
    let my_box = ShippingBox{
        depth: 3,
        width: 4,
        height: 5,
    };
    let tall = my_box.height;
    println!("the box is {tall} cm tall");
    println!("this is another measure in feet: {:?}", my_box.depth);
}