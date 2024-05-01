struct User {
    active: bool,
    name: String,
    email: String,
}
//tuple struct
struct Colour(i32, i32, i32);

struct Rectangle {
    width: i32,
    height: i32,
}
impl Rectangle {
    fn area(&self) -> i32 {
        self.height * self.width
    }
}
fn main() {
    let user = User {
        active: true,
        name: String::from("Eshan"),
        email: String::from("hfasd@email.com"),
    };
    println!(
        "name is {},email is {} and the user is active:{}",
        user.name, user.email, user.active
    );

    // let black: Colour = Colour(0, 0, 0);
    // println!("{}", black);
    let rect = Rectangle {
        width: 12,
        height: 12,
    };
    println!("{}", rect.area());
}
