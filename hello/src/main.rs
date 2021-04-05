use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, 100 days of Rust");
    let target = rand::thread_rng().gen_range(1..=9);

    loop {
        let mut guess = String::new();

        //read a line from stdin
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");

        // convert input to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // println!("{}", guess);
        println!("Your guess is:{}", guess);
        
        match guess.cmp(&target) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}