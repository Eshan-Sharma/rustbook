fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest_num = largest(&number_list);
    println!("Largest number is {}", largest_num);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest_num = largest(&number_list);
    println!("Largest number is {}", largest_num);
}

fn largest(list: &Vec<i32>) -> &i32 {
    let mut largest_num: &i32 = &list[0];
    for number in list {
        if number > largest_num {
            largest_num = number;
        }
    }
    largest_num
}
