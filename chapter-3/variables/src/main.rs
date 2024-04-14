// fn main() {
//     let x = 5;
//     println!("The value of x is : {x}");
//     x = 6; // immutable variable it throws an error
//     println!("The value of x is : {x}");
// }

// fn main() {
//     let mut x = 5; //mutable variable and can be changed
//     println!("The value of x is : {x}");
//     x = 6;
//     println!("The value of x is : {x}");
//     const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60; //const
//     println!("3 hours in seconds {}", THREE_HOURS_IN_SECONDS);
// }

fn main() {
    let x = 5;
    let x = x + 1; //operation on outer x
    {
        let x = x * 2; //operation on outer x after x+1
        println!("The value of x in the inner scope {x}");
    } //inner operation is destroyed coming out of scope
    println!("The value of x outside scope {x}"); //only x+1 remains
}
