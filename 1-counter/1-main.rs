use std::error::Error;
use std::io;

fn main() {
    println!("enter number");
    match parse_number() {
        Ok(mut number) => loop {
            println!("enter command");
            let mut command = String::new();
            io::stdin()
                .read_line(&mut command)
                .expect("failed to read command");
            match command.trim() {
                "inc" => {
                    number += 1;
                    println!("incremneted number{}", number);
                }
                "dec" => {
                    number -= 1;
                    println!("decremented number {}", number);
                }
                "exit" => {
                    println!("exiting program");
                    break;
                }
                _ => println!("Unknown command Please enter 'inc' or 'dec' or 'exit'."),
            }
        },

        Err(e) => println!("error is{}", e),
    }
}

fn parse_number() -> Result<i32, Box<dyn Error>> {
    let mut numm = String::new();
    io::stdin().read_line(&mut numm)?;
    let fdfd = numm.trim().parse()?;
    Ok(fdfd)
}
