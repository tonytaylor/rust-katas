extern crate chrono;

use std::io;
use chrono::prelude::*;

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

fn years_to_work(x: i32, y: i32) -> i32 {
    x - y
}

fn year_done(x: i32, y: i32) -> i32 {
    x + y
}

fn prepare_output(ytr: i32, current_yr: i32) -> String {
    format!("you have {} years until you retire \n\
    It's {}, you retire in {}.", ytr, current_yr, year_done(current_yr, ytr))
}

fn main() {
    let curr_age = get_input("What is your current age?");
    let retire_age = get_input("At what age would you like to retire?");
    println!("{}", prepare_output(years_to_work(retire_age, curr_age), Local::now().year()))
}