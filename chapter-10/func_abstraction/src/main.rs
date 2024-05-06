struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}
fn main() {
    {
        let integer = Point { x: 1, y: 3 };
        println!("integer.x is {}", integer.x);
        let float = Point { x: 2.3, y: 9.0 };
        //wont work if we have different in x and y data types if only Point<T> used
        let wont_work = Point { x: 1, y: 9.0 }; //Works with Point<T,U> used
    }
    {
        let number_list = vec![34, 50, 25, 100, 65];
        let largest_num = largest(&number_list);
        println!("Largest number is {}", largest_num);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        let largest_num = largest(&number_list);
        println!("Largest number is {}", largest_num);

        let char_list = vec!['q', 'd', 'c', 'a', 'z', 'n'];
        let largest_character = largest(&char_list);
        println!("Largest character is {}", largest_character);
    }
}

// fn largest(list: &Vec<i32>) -> &i32 {
//     let mut largest_num: &i32 = &list[0];
//     for number in list {
//         if number > largest_num {
//             largest_num = number;
//         }
//     }
//     largest_num
// }
// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];
//     for character in list {
//         if character > largest {
//             largest = character;
//         }
//     }
//     largest
// }
fn largest<T: std::cmp::PartialOrd>(list: &Vec<T>) -> &T {
    let mut largest_element = &list[0];
    for element in list {
        if largest_element < element {
            largest_element = element;
        }
    }
    largest_element
}
