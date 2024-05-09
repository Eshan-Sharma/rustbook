// enum List {
//     Cons(i32, Box<List>), // Rust knows how much space does Box<T> takes since it is a pointer
//     Nil,
// }
use std::rc::Rc;
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    // Infinite Storage
    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }

    //Box implementation
    // let list = List::Cons(
    //     1,
    //     Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    // );
    //Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities, like those we’ll see with the other smart pointer types.

    //Rc implementation
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    let b = List::Cons(3, Rc::clone(&a));
    let c = List::Cons(4, Rc::clone(&a)); // value moved error
}
