use std::collections::HashMap;

fn main(){
    let mut map = HashMap::new();

    map.insert("Petr".to_string(), "Programmer".to_string());
    map.insert("Leon".to_string(), "Son of Petr".to_string());
    map.insert("Sergey".to_string(), "Father of Petr".to_string());

    println!("{:?}", map);


    

}