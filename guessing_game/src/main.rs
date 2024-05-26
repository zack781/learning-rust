use std::io;
use rand::prelude::*;

fn main() {
    println!("A random number has been generated!");

    let number: u8= random();

    println!("Please Enter Your Guess: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line!");

    println!("Random Number is {}", number);
}
