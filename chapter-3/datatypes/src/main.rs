fn main() {
    println!("Hello, world! just printing data types");
    let x: f32 = 3.02; //f32
    println!("x is {}", x);

    let y = 4.01; //f64
    println!("y is {}", y);

    let sum = 5 + 10;
    println!("sum is {sum}"); //int

    let diff = 34.2 - 21.02;
    println!("diff is {diff}"); //float - this shows an interesting property as well of floating point error

    let quotient = 543.21 / 89.1;
    println!("quotient is {quotient}"); //float divided by float
    let truncated = -1 / 9;
    println!("truncated is {truncated}"); // int divided by int

    let remainder = 10 % 3;
    println!("remainder is {remainder}");

    let c: char = 'q';
    println!("character is {c}"); //char

    let babe: (u32, char, f32) = (123, 'd', 4.54);
    println!("babe is {},{},{}", babe.0, babe.1, babe.2);

    let arr = [1, 2, 4, 5, 8];
    let months = [
        "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
    ];
    let arr1 = [i32; 5];
    println!("array is ");
}
