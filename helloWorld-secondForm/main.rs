use std::io;

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_var) => {},
        Err(_no_updates_at_this_point) => {}
    }
    input.trim().to_string()
}

fn assemble_greeting(name: String) -> String {
    format!("Hello, {}, nice to meet you!", name)
}

fn print_greeting(greeting: String) {
    println!("{}", greeting)
}

fn main() {
    print_greeting(assemble_greeting(get_input("What is your name?")))
}