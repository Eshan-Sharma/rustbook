fn main() {
    println!("Hello, world!");
    //print_name();
    // sum(3, 4);
    // print_labelled_measurements(5, 'm');
    // statements_expressions();
    func_with_return();
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

fn print_labelled_measurements(value: i32, unit_label: char) {
    println! {"The measurement is {value}{unit_label}"};
}

fn statements_expressions() {
    let y = 5;
    let a = {
        let x = 3; //this statement alone does not have any return value
        x //this acts as a return value for the scope and is assigned to "a"
    }; //here semicolon is important
    println!("a is {a}");
}

fn func_with_return() {
    fn five() -> i32 //type of return
    {
        5
    } //returns a i32 value 5
    let x = five();
    println!("x is {x}");
}
