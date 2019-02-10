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

fn get_char_count(subject: String) -> usize {
    subject.chars().count()
}

fn print_result(subject: String, word_count: usize) {
    println!("{} has {} characters.", subject, word_count)
}

fn main() {
    let input = get_input("What is the input string?");
    print_result(input.clone(), get_char_count(input))
}