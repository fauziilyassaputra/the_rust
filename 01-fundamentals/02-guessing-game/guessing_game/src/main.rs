// use std::io;

// fn main() {
//     println!("Guess the Number!");

//     println!("Please Input your guess");

//     let mut guess = String::new();

//     io::stdin()
//     .read_line(&mut guess)
//     .expect("Failed to read line");

//     println!("You guess {guess}");
// }


// use std::io;

// // use rand::Rng;

// fn main() {
//     println!("Guess the Number!");
    
//     let secret_number = rand::random_range(1..=100);

//     println!("The Secret Number is: {secret_number}");

//     println!("Please Input your guess");

//     let mut guess = String::new();

//     io::stdin()
//     .read_line(&mut guess)
//     .expect("Failed to read line");

//     println!("You guess {guess}");
// }

use std::io;
use std::cmp::Ordering;
// use rand::Rng;

fn main() {
    println!("Guess the Number!");
    
    let secret_number = rand::random_range(1..=100);

    loop {
    println!("The Secret Number is: {secret_number}");

    println!("Please Input your guess");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guess {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!!"),
        Ordering::Greater => println!("To Big!"),
        Ordering::Equal => {
            println!("you win");
            break;
        }
    }

    }
   
}
