use std::io;

fn weight_on_mars() {
    println!("please enter your input weight");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error reading input");

    match input.trim().parse::<f32>() {
        Ok(value) => {
            let fs = (value / 9.81) * 3.711;
            println!("weight on mars{}", fs);
        }

        Err(e) => println!("please enter valid number: {}", e),
    }
}

fn main() {
    loop {
        weight_on_mars();
    }
}
