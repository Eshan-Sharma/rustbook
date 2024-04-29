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
    println!("---------------------------------");
    lift_off();
    println!("---------------------------------");
    iterator_loop();
    println!("---------------------------------");
    println!("32 Fehrenheit is {} celsius",fahrenheit_to_celsius(32.0));
    println!("---------------------------------");
    println!("0 Celsius is {} fahrenheit",celsius_to_fahrenheit(0.0));

}
fn lift_off(){
    let mut number = 3;
    // while number!=0{
    //     println!("{number}");
    //     number-=1;
    // }
    // println!("Lift off");

    //Using for a reverse a range(rev)
    for number in (1..4).rev(){
        println!("Countdown {number}");
    }
    println!("Lift off!");

}
fn iterator_loop(){
    let a = [1,2,3,4,5];
    let mut index = 0;
    // unoptimized
    // while index<5{
    //     println!("The value is {}",a[index]);
    //     index+=1;
    // }
    //Optimized
    for element in a{
        println!("Value is {element}");
    }
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

fn fahrenheit_to_celsius(x:f64)-> f64{
let celsius = (x-32.0)/1.8;
celsius
}

fn celsius_to_fahrenheit(x:f64)->f64{
let fahrenheit = (x*9.0)/5.0 + 32.0;
fahrenheit
}