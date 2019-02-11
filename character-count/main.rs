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

fn get_char_count(subject: String) -> usize {
    subject.chars().count()
}

fn assemble_output(subject: String) -> String {
    format!("{} has {} characters", subject.clone(), get_char_count(subject))
}

fn print_result(subject: String) -> () {
    println!("{}", subject)
}

fn main() {
    print_result(assemble_output(get_input("What is the input string?")))
}