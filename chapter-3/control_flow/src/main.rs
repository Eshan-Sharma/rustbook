fn main() {
    println!("Enter your age");
    let mut age = String::new();
    std::io::stdin()
        .read_line(&mut age)
        .expect("failed to read line");

    let trimmed = age.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => {
            if i > 18 {
                println!("You are adult.");
            } else {
                println!("You are a minor.")
            }
        }
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
}
