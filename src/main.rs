use std::io;

fn ask_question(question: &str) -> String {
    println!("{question}");
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Oops! Looks like there be a sea monster in the I/O waters.");

    input.trim().to_string()
}

fn main() {
    // A warm welcome
    println!("🌟 Welcome to the Rustic Adventure! 🚀");

    // Asking for the adventurer's name
    let name = ask_question("What do they call ye, brave coder?");

    // Inquiring about the adventurer's age
    let age = ask_question("How many orbits around the sun have ye completed?");

    // Unveiling the personalized greeting
    println!("🎉 Ahoy, {name}! Ready to set sail into the Rustic seas\nof coding at the youthful age of {age}? 🚢⚓");
}
