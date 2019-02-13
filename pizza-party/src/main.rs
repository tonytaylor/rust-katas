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
        Err(e) => panic!("IO Read Error: {}", e)
    }
}

fn prepare_output(head_count: i32, pizza_count: i32, slices_per_pizza: i32) -> String {
    format!("{} people with {} pizzas \n\
    Each person gets {} slices of pizza \n\
    There are {} slices leftover,", head_count, pizza_count, 
    calculate_slices_per_person(head_count, pizza_count, slices_per_pizza),
    get_slice_remainder(head_count, pizza_count, slices_per_pizza))
}

fn calculate_slices_per_person(head_count: i32, pizza_count: i32, slices_per_pizza: i32) -> i32 {
    slices_per_pizza / (head_count / pizza_count)
}

fn get_slice_remainder(head_count: i32, pizza_count: i32, slices_per_pizza: i32) -> i32 {
    slices_per_pizza % (head_count / pizza_count)
}

fn main() {
    let head_count = get_input("How many people?");
    let pizza_count = get_input("How many pizzas do you have?");
    let slices_per_pizza = get_input("How many slices per pie?");
    println!("{}", prepare_output(head_count, pizza_count, slices_per_pizza));
}
