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

fn get_sq_footage(x: i32, y: i32) -> i32 {
    x * y
}

fn convert_to_meters(x: i32) -> f32 {
    const TO_METERS: f32 = 0.09290304;
    x as f32 * TO_METERS
}

fn prepare_output(area: i32, area_in_meters: f32) -> String {
    format!("The area is \n{} square feet\n{:.*} square meters", area, 3, area_in_meters)
}

fn main() {
    let rm_length = get_input("What is the length of the room in feet?");
    let rm_width = get_input("What is the width of the room in feet?");
    let area_in_sq_ft = get_sq_footage(rm_length, rm_width);

    println!("{}", prepare_output(area_in_sq_ft, convert_to_meters(area_in_sq_ft)));
}
