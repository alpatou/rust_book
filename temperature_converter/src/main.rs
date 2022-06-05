use std::io;

fn main() {
    print!("
        Hello, your options are these:
        1. Celsius to Fahrenneits 
        2. Fahrenneits to Celsium
    ");

    let mut mode = String::new();

    io::stdin().read_line(&mut mode).expect("l");
}

//(30°C x 1.8) + 32 = 86°       fn
//(50°F - 32) x .5556 = 10°C

fn celsius_to_farhenneit(temp_in_celsius: f32) -> f32 {
    (temp_in_celsius * 1.8 ) + 32 as f32
}

fn fahrenneits_to_celsius(temp_in_fahren: f32) -> f32 {
    let subtract_factor:f32 = 32.0;
    let multiplication_factor = (5/9) as f32;
    (temp_in_fahren -  subtract_factor) * multiplication_factor
}
