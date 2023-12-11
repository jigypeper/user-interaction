use std::io;

fn main() {
    println!("Whats your name?");
    let mut input: String = String::new();
    let mut input_age: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    let name = input.trim();
    println!("How old are you?");
    io::stdin()
        .read_line(&mut input_age)
        .expect("Failed to read line!");
    let age = input_age.trim();
    println!("Hello {name}! You are {age} years old");
}
