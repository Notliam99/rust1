use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};

fn main() {
    execute!(io::stdout(), EnterAlternateScreen).expect("Error: Couldnt Enter Alternitive Screen");

    let mut name = String::new();

    print!("hello whats your name\n> ");

    io::stdout().flush().unwrap();

    execute!(
        io::stdout(),
        SavePosition,
        MoveTo(10, 10),
        EnableBlinking,
        DisableBlinking,
        RestorePosition
    )
    .expect("ERRor couldnt work");

    io::stdin()
        .read_line(&mut name)
        .execute(MoveTo(11, 11))
        .expect("Error: Could Not Read Line");

    execute!(io::stdout(), LeaveAlternateScreen).expect("Error: Couldnt Exit Alternitive Screen");

    println!("Hello {name}");
}
