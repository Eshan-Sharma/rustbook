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
    {
        let v1 = vec![1, 2, 3];
        //v1.iter().map(|x| x + 1)  // iterators are a lazy function and do nothing unliss consumed
        let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
        print!("value after map is {:?}", v2);
    }
}
