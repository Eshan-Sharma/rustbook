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
    multiple_loop();
}

fn multiple_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println! {"remaining is {remaining}"};
            if (remaining == 9) {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
