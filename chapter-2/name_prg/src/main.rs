fn main() {
    println!("Addition");
    println!("Enter first name");
    let mut firstname = String::new();
    std::io::stdin()
        .read_line(&mut firstname)
        .expect("Error reading line");
    println!("Enter last name");
    let mut lastname = String::new();
    std::io::stdin()
        .read_line(&mut lastname)
        .expect("Error reading line");
    println!("Name is {}", firstname + &mut lastname);
}
