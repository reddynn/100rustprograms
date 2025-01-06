use std::error;
use std::f32;
use std::io;

fn weight_on_mars() -> Result<f32, Box<dyn error::Error>> {
    println!("please enter your weight");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input_to_number = input.trim().parse::<f32>()?;
    let weight = (input_to_number / 9.81) * 3.711;
    Ok(weight)
}

fn main() {
    loop {
        let weight_m = weight_on_mars();
        match weight_m {
            Ok(value) => println!("Your weight on Mars is: {:.2} kg", value),
            Err(e) => println!("error is{}", e),
        }
    }
}
