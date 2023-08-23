use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("## Guess the number ##");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("secret number = {secret_number}");

    loop {
        println!("Enter a guess:");

        let mut guess: String = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Readline failed");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        }
    
    }

}
