# Rustic Adventures: Navigating Console Interaction in Rust! 🚀

Greetings, fellow technologists! Today marks my inaugural foray into technical blogging, 
I've chosen to do it with the Rust programming language, and what better way to immerse 
myself than to share the journey of building a console application. As I navigate these 
Rustic waters, I'll delve into the technical nuances of user interaction, exploring the 
language's syntax and features.

## A Technical Maiden Voyage

This post isn't just an expression of my curiosity; it's a deliberate choice to deepen 
my understanding of Rust by building tangible projects. I've chosen the realm of console 
applications, where Rust's emphasis on memory safety and performance shines. Our vessel 
for this maiden voyage is a simple yet instructive program that engages users in a dynamic 
dialogue.

### Breakdown
The idea is to ask the user for their name, age, and then display an output in the console.

While Python offers a vast standard library that abstracts away many lower-level details, 
Rust's approach of keeping these details visible encourages developers to think more 
critically about the programs they write.

The first thing to do is ask the user a question:
```rust
// we will need 'io' from the standard
// library
use std::io;

fn main() {
  // using the print line macro is the
  // quickest and easiest way to display
  println!("What is your name?");

  // Create a mutable string variable called input  
  let mut input: String = String::new();

  // Get the input from the standard input (stdin)
  // Read a line of text from stdin and store it in the input variable
  // Check if the read operation was successful
  io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line!");

  // Display the trimmed input to the user
  println!("Hello {}!", input.trim());
   
}
```
If you come from a Python background, you'll be wondering why it isn't
as simple as just calling the input function.

```python
name = input("what is your name? ")
print(f"Hello {name}!")
```
Two lines vs 4 is a no brainer. This is where we start to think about abstraction.
We can create our own 'ask_question' function that behaves like the Python one.

```rust
fn ask_question(question: &str) -> String {
    println!("{question}");
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Oops! Looks like there be a sea monster in the I/O waters.");

    input.trim().to_string()
}
```
