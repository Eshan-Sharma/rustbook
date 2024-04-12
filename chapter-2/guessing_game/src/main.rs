// use std::io;

// fn main() {
//     println!("Guess the number");
//     println!("Please input your guess");
//     let mut guess = String::new(); //Mutuable -> mut
//                                    //String::new() -> empty string
//                                    //variables in rust are immutable(const) by default

//     io::stdin() //can use io::stdin or std::io::stdin() without use std::io
//         .read_line(&mut guess) //Passing as an argument, tells readline where to store user input
//         //& is reference -> references are also immutable by default so used &mut guess
//         .expect("Failed to read line"); //error handling

//     println!("You guessed : {guess}");
// }

use rand::Rng; //random number generators implement Rng
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=10); //rand::thread_rng function gives particular random number generator, that is local to the current thread of execution seeded by Operating system
                                                              // gen_range takes the range expression as argument and generates random number in range
                                                              //start..=end this is inclusive lower and upper bound
    println!("Secret number is {secret_number}");
    println!("Please input your guess number");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading line");

    match guess.cmp(&secret_number){
        Ordering::Less=>println!("Too less");
        Ordering::Greater=>println!("Too big");
        Ordering::Equal=>println!("You win!");
    }

    println!("Your guess is {}", guess);
}
