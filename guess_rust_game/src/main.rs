#![deny(clippy::all)]
use std::io;

fn main() {
    println!("Welcome to the guess game ");
    println!("Enter the number :");

    let mut num=String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    println!("The number you guess is {}",num);

}
