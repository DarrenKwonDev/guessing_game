use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // 해당 thread 내에서만 사용하는 rand 생성
    let secret_num = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_num}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // guess가 num이됨
            Err(_) => {
                continue;
            }, 
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),  // guess가 secret_num보다 작을 때
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}