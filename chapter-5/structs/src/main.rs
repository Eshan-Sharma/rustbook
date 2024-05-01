//classic struct
struct User {
    active: bool,
    name: String,
    email: String,
}
//tuple struct
struct Colour(i32, i32, i32);

//Unit struct
struct CalculateArea;
struct Rectangle {
    width: i32,
    height: i32,
}
impl Rectangle {
    fn area(&self) -> i32 {
        self.height * self.width
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}
fn main() {
    //classic struct
    let user = User {
        active: true,
        name: String::from("Eshan"),
        email: String::from("hfasd@email.com"),
    };
    println!(
        "name is {},email is {} and the user is active:{}",
        user.name, user.email, user.active
    );

    //tuple struct
    let black = Colour(0, 0, 0);
    println!("{},{},{}", black.0, black.1, black.2);

    let rect = Rectangle {
        width: 12,
        height: 12,
    };
    println!("{}", rect.area());

    let my_direction = Direction::East;
}
