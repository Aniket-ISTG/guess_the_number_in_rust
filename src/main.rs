use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop{
        println!("Please input your guess number");

        let mut guess: String =  String::new(); 
        io::stdin().read_line(&mut guess).expect("Failed to readline");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num, 
            Err(_) => continue // (_) means match with anything
        };

        // guess currently holds an empty string
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("TOO SMALL"),
            Ordering::Greater => println!("TOO BIG"),
            Ordering::Equal => {
                println!("You Win");
                println!("quit");
                break;
            },
        }
        
    }
}
