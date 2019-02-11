use std::io;

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            if n < 2 {
                get_input("please enter at least one character")
            } else {
                input.trim().to_string()
            }
        },
        Err(e) => {
            panic!("Error: {}", e);
        }
    }
}

fn prepare_quote(raw_quote: String) -> String {
    String::from(raw_quote)
}

fn main() {
    println!("{} says {}", 
        get_input("Who said it?"), prepare_quote(get_input("What is the quote?")));
}

