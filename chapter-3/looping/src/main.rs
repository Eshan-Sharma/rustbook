fn main() {
    let mut x = 1;
    loop {
        println!("again!");
        x += 1; //Check to make sure the loop is not infinite
        if (x > 3) {
            break;
        }
    }
    println!("---------------------------------");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
    println!("---------------------------------");
   
}
