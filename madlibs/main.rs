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

fn prepare_output(noun: String, verb: String, adjective: String, adverb: String) -> String {
    format!("Do you {} your {} {} {}? That's hilarious!", verb, adjective, noun, adverb)
}

fn main() {
    let noun = get_input("Enter a noun:");
    let verb = get_input("Enter a verb:");
    let adj = get_input("Enter an adjective:");
    let adv = get_input("Enter an adverb:");
    println!("{}", prepare_output(noun, verb, adj, adv));
}