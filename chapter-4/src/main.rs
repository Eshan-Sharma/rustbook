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
    println!("{str2}"); //shallow copy, but since str1 pointer is "moved" to str2. The process is "move"
                        //Deep copies are not automatically created
                        //this type of copy is inexpensive in terms of runtime performance

    let str_to_be_copied = String::from("copy me!");
    let str_which_copied = str_to_be_copied.clone(); //Creates a deep copy, but this has to be expicitly mentioned as "clone"
    println!(
        "string to be copied is: {}\n and String copied is: {}",
        str_to_be_copied, str_which_copied
    );
    ownership();
}

//scope end s is not valid

fn ownership() {
    let str = String::from("hwllo"); //str is here
    takes_ownership(str); //str is passed, ownership is passed'
                          //println!("{str}");//This will give error since str no longer valid here
    let x = 5;
    makes_copy(x); //copy of x is passed
    println!("x after function: {x}");

    let str1 = String::from("take and give back");
    let str2 = gives_ownership(str1);
    println!("str2 after: {str2}");
    //println!("str1 after: {str1}");//gives error\
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
fn gives_ownership(sone_string: String) -> String {
    println!("Some string inside gives back ownership: {sone_string}");
    sone_string
}
