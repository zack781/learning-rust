
pub struct Node {
    data: i32,
    prev: Option<Box<Node>>,
    next: Option<Box<Node>>,
}

// impl Node {

// }

fn take_ownership_of_string(str: String) {
    println!("str = {}", str);
}

fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // take_ownership_of_string(s2);
    println!("{}, world!", s2);


}
