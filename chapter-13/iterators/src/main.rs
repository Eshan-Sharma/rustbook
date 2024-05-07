fn main() {
    {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        println!("Iterator v1_iter is called");
        for item in v1_iter {
            println!("Value is {}", item);
        }
        println!("v1_iter iterator is used up calling does not work");
    }
}
