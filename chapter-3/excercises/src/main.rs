fn main() {
    temp_convertor();
    fibonacci();
}
fn fibonacci() {
    println!("Enter a number for nth fibonacci");
    let mut fib_num = String::new();
    std::io::stdin()
        .read_line(&mut fib_num)
        .expect("Enter a number");
    let fib_num: u32 = match fib_num.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Please enter a positive number"),
    };
    println!(
        "The {}th fibonacci number is {}",
        fib_num,
        nth_fibonacci(fib_num - 1)
    );

    fn nth_fibonacci(mut n: u32) -> u32 {
        if n == 1 {
            return 1;
        } else if n == 0 {
            return 0;
        }
        n = nth_fibonacci(n - 1) + nth_fibonacci(n - 2);
        return n;
    }
}
fn temp_convertor() {
    println!("Please enter a temperature in fahrenheit");
    let mut temperature = String::new();
    std::io::stdin()
        .read_line(&mut temperature)
        .expect("Enter a number");
    let temperature: f64 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Enter a valid temperature"),
    };
    println!("{}", fahrenheit_to_celcius(temperature));
    //
    println!("Please enter a temperature in celcius");
    let mut temperature = String::new();
    std::io::stdin()
        .read_line(&mut temperature)
        .expect("Enter a number");
    let temperature: f64 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Enter a valid temperature"),
    };
    println!("{}", celcius_to_fahrenheit(temperature));
}

fn fahrenheit_to_celcius(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

fn celcius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0) / 5.0 + 32.0
}
