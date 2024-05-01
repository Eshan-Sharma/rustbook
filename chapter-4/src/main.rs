fn main() {
    // s will give a warning at compile time, use _s.
    // _ tells the compiler that it is intentional, compiler need not worry about it (we know what we are doing)
    let _s = "hello"; //string literal, value is hard coded into the text of program.
                      //s is valid here

    let str = String::from("Hello world!");
    println!("{str}");

    let mut s1 = String::from("hello"); //String literal, defined mutable
    s1.push_str("world"); //we can mutable a String literal
    println!("{s1}");

    //Integers are simple values and stored on the stack
    //y is copy of x, y = 4
    let x = 4;
    let y = x;
    println!("x: {} y: {}", x, y);
    //
    let str1 = String::from("string value");
    let str2 = str1;
    //println!("{str1}");//This will return an error since the pointer to the value of str1 has been moved to the str2
    println!("{str2}"); //shallow copy

    //
    let str_to_be_copied = String::from("copy me!");
    let str_which_copied = str_to_be_copied.clone(); //Creates a deep copy, but this has to be expicitly mentioned as "clone"
    println!(
        "string to be copied is: {} and String copied is: {}",
        str_to_be_copied, str_which_copied
    );
}

//scope end s is not valid
