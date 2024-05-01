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
    let manager = build_manager(String::from("manager@email.com"), String::from("boss420"));
    println!(
        "manager email is: {}, status is active:{}",
        manager.email, manager.active
    );
    let manager2 = Manager {
        email: String::from("awesomemanager@email.com"),
        username: String::from("ManagerIsMe"),
        active: manager.active, //Update syntax to create new instance of struct from another instance
        sign_in_count: 1,
    };
    println!(
        "manager2 email is: {}, status is active:{}, username: {}",
        manager2.email, manager2.active, manager2.username
    );

    // //tuple struct
    // let black = Colour(0, 0, 0);
    // println!("{},{},{}", black.0, black.1, black.2);

    // let rect = Rectangle {
    //     width: 12,
    //     height: 12,
    // };
    // println!("{}", rect.area());

    // let my_direction = Direction::East;
}

struct Manager {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u32,
}
fn build_manager(email: String, username: String) -> Manager {
    Manager {
        email,              //rust short hand
        username: username, //without rust shorthand
        active: true,
        sign_in_count: 1,
    }
}
