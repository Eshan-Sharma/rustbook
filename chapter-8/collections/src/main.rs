use std::vec;

fn main() {
    let mut v: Vec<i32> = Vec::new(); //creating a new vector
    v.push(2); //updating a vector
    v.push(3);
    let second_reference = &v[1]; //Value can be read by a reference
    let second_option = v.get(1); //or by get method
    println!("{}", &v[1]); //If a index is out of bound the program will give an error, this exists if you delibrately want it to crash
                           //{:?} printing using debug
    println!("{:?},{}", second_option, second_reference);
    //Handling Option<i32>
    match second_option {
        Some(second_option) => println!("{}", second_option),
        None => println!("None"),
    }
    let _v1 = vec![1, 2, 32]; //vector declaration using macros
    let _v2 = vec!["S", "asd", "sad"]; //rust infers the type, all values should be the same type!

    {
        let mut v = vec![1, 2, 3, 4, 5];
        v.push(6); //but this is valid, mutate before immutate borrow
        let first = &v[0];

        // v.push(6);//error, mutating after immutable borrow

        println!("The first element is: {first}"); //immutable borrow later used here
    }
    {
        //Iterating over vector
        let v = vec![1, 2, 3];
        for i in &v {
            println!("Original vector - {}", i);
        }
    }
    {
        let mut v = vec![1, 2, 3];
        for i in &mut v {
            *i *= 2; //derefernce to get the original value then multiply by 2
            println!("Vector changed - {}", i);
        }
    }
    {
        //If you know at runtime the exhaustive set of types the program will get at runtime you can use enum
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f32),
            Text(String),
        }
        let vector_of_different_types = vec![
            SpreadsheetCell::Int(2),
            SpreadsheetCell::Float(12.3),
            SpreadsheetCell::Text(String::from("Hello")),
        ];
        for i in &vector_of_different_types {
            println!("{:?}", i);
        }
    }
    {
        let mut s = String::new();
        let data = "initial content";
        //creating a String from a string literal
        let string_data = data.to_string();
        let s = data.to_string();

        let mut greet = String::from("नमस्ते");
        greet.push_str(" world"); //updating string
        println!("{}", greet);

        let s1 = String::from("yo yo ");
        let s2 = String::from("honey singh!");
        let s3 = s1 + &s2; //concatenation using + operator, s1 is moved here and s2 is referenced
        println!("{s3}");
    }
}
