use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Welcome to File Handling");
    println!("0: Create a new file");
    println!("1: Edit an existing file");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Invalid Input");
    let option: u8 = input.trim().parse().expect("Invalid Input");
    input = "".to_string();

    match option{
        0 => {
            println!("Creating a new file");
            println!("Enter the filename: ");
            io::stdin().read_line(&mut input);
            let mut file = File::create(input.trim_end());
            input = "".to_string();

            println!("Enter the content of the file: ");
            io::stdin().read_line(&mut input);
            match file {
                Ok(mut f) => {
                    f.write_all(input.as_bytes());
                    input = "".to_string();
                }
                Err(e) => {
                    println!("Error");
                }
            }
        }
        1 => {
            println!("Editing an exting file");
            println!("Enter the file name:");
            io::stdin().read_line(&mut input);
            let mut file = File::open(input.trim_end())?;
            input = "".to_string();
            let mut content = String::new();
            file.read_to_string(&mut content);
            println!("File content is: {}", content);
        }
        _ => {
            println!("None");
        }
    }

    Ok(())
}
