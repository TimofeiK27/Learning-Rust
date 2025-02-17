use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("INPUT");

   
    let secret_number = rand::thread_rng().gen_range(1,10);


    loop {
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    

        let guessint: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your Guess was : {}", guessint);

        match guessint.cmp(&secret_number) {
            Ordering::Less => println!("Less"),
            Ordering::Equal => {println!("Equal"); break;},
            Ordering::Greater=> println!("Greater"),
        }
    }
    
}
