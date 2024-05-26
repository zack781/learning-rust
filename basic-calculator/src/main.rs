use std::io;

fn main() {
    println!("Welcome to Basic Calculator");
    println!("0: Addition");
    println!("1: Subtraction");
    println!("2: Multiplication");
    println!("3: Division");

    println!("Select Your Operation: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input);

    let chosen_opr: u8 = input.trim().parse().expect("Input is not an integer");

    input = "".to_string();
    println!("Enter your first number (a): ");
    io::stdin().read_line(&mut input);
    let first_num: u8 = input.trim().parse().expect("Not a valid number");

    input = "".to_string();
    println!("Enter your second number (b): ");
    io::stdin().read_line(&mut input);
    let second_num: u8 = input.trim().parse().expect("Not a valid number");

    match chosen_opr {
        0 => {
            println!("You chose addition");
            println!("a + b = {}", first_num + second_num);
        }
        1 => {
            println!("You chose subtraction");
            println!("a - b = {}", first_num - second_num);
        }
        2 => {
            println!("You chose multiplication");
            println!("a * b = {}", first_num * second_num);
        }
        3 => {
            println!("You chose division");
            println!("a / b = {}", first_num / second_num);
        }
        _ => {
            println!("Invalid choice");
        }
    }
}
