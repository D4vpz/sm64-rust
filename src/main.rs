use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Geuss the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your geuss.");

        let mut geuss: String = String::new();

        io::stdin()
            .read_line(&mut geuss)
            .expect("Failed to read line");

        let geuss: u32 = match geuss.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Thats not a number!");
                continue;
            }
        };

        println!("You geussed: {geuss}");

        match geuss.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
