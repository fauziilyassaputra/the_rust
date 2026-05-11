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


use std::io;

// use rand::Rng;

fn main() {
    println!("Guess the Number!");
    
    let secret_number = rand::random_range(1..=100);

    println!("The Secret Number is: {secret_number}");

    println!("Please Input your guess");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    println!("You guess {guess}");
}
