fn main() {
    println!("Hello, world!");
    //print_name();
    sum(3, 4);
}

fn print_name() {
    println!("Enter your name");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Enter name");
    println!("Your name is {name}");
}

fn sum(x: u32, y: u32) {
    //unsigned 32
    //the data types are defined here and not while passing the variables,
    //this is similar to other languages like java (for own understanding)
    println!("x + y is {}", x + y);
}
