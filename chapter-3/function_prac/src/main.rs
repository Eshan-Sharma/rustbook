fn main() {
    println!("Hello, world!");
    print_name();
}

fn print_name() {
    println!("Enter your name");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Enter name");
    println!("Your name is {name}");
}
