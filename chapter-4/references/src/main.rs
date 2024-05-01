fn main() {
    let mut s = String::from("hello");
    change(&mut s);

    fn change(some: &mut String) {
        //Reference should be mutable
        some.push_str("world");
    }

    //let r1 = &mut s;
    //let r2 = &mut s; //cannot be mutable twice, this prevents race condition from occuring
    //println!("{},{}", r1, r2);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
                 //let r3 = &mut s;//error
                 //immutable reference and mutable references cannot be used together
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);

    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);
}
