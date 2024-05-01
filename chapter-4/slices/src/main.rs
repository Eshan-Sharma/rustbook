fn main() {
    let s = String::from("hello world");
    let word = first_world(&s);
    println!("{word}");
}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// fn second_word(s: &String) -> (usize, usize) {
//     let bytes = s.as_bytes();
//     let mut first_index: usize = 0;
//     let mut found_first: bool = false;

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' && !found_first {
//             first_index = i + 1;
//             found_first = true;
//         } else if item == b' ' && found_first {
//             return (first_index, i - 1);
//         }
//     }
//     (s.len(), s.len())
// }
