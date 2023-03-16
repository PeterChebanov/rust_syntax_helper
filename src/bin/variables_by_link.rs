
//General 2 rules of using links:
// ---> in one moment it can be a lot of unmutubale links (read only)
// ---> in one moment it can be only one mutable link (has write permissions)


fn main() {

    let mut s1 = "hello".to_string();

    s1 = get_ownership(s1); // if we do not save the result to s1, the ownership of the variable will be passed to fn get_ownership()
    println!("s1: {s1}");

    let tuple = get_ownership_return_len(s1);
    println!("len {}", tuple.0);

    s1 = tuple.1;
    println!("s1 : {}", s1);

    let mut elon = "Elon Mask".to_string();
    let ref_em = &elon;

    Siberia(ref_em);


    let mut_elon = &mut elon;
    elon_talk(mut_elon);
    println!("elon : {}", elon);


}

fn get_ownership(s: String) -> String {

    println!("s : {s}");
    s
}

fn get_ownership_return_len(s: String) -> (usize, String) {
    (s.len(), s)
}

fn Siberia(p: &String) {
    println!("I'm {p}, and I'm now in Siberia");
}

fn elon_talk(p: &mut String) {
    p.push_str("Buy sell dodge, bitcoin!! HAHAHA");
}