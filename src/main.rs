use std::io;
use std::io::Write;

fn main() {
    let x = get_user_input("User input: ".to_string());
    println!("{}", x);
}

// Gets user input as a cool independent function
fn get_user_input(inp: String) -> String {
    print!("{}", inp);
    io::stdout().flush();

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read");

    answer
}