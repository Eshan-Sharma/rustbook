use std::io;

fn main() {
    println!("Guess the number");
    println!("Please input your guess");
    let mut guess = String::new(); //Mutuable -> mut
                                   //String::new() -> empty string
                                   //variables in rust are immutable(const) by default

    io::stdin() //can use io::stdin or std::io::stdin() without use std::io
        .read_line(&mut guess) //Passing as an argument, tells readline where to store user input
        //& is reference -> references are also immutable by default so used &mut guess
        .expect("Failed to read line"); //error handling

    println!("You guessed : {guess}");
}
