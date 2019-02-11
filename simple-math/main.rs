use std::io;

fn get_input(prompt: &str) -> i32 {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            if n < 2 {
                get_input("please enter at least one character")
            } else {
                match input.trim().parse::<i32>() {
                    Ok(m) => m,
                    Err(_) => get_input("please enter a number")
                }
            }
        },
        Err(e) => {
            panic!("IO Read Error: {}", e);
        }
    }
}

fn prepare_output(x: i32, y: i32) -> String {
    format!("{val1} + {val2} = {}\n{val1} - {val2} = {}\n{val1} * {val2} = {}\n{val1} / {val2} = {}", 
    x+y, x-y, x*y, x/y, val1=x, val2=y)
}


fn main() {
    let x = get_input("What's the first number?");
    let y = get_input("What's the second number?");
    println!("{}", prepare_output(x, y));
}