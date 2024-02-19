use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("hello whats your name\n> ");

    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Error: Could Not Read Line");
    print!("{esc}c", esc = 27 as char);

    println!("Hello {input}");
}
