enum Direction {
    North,
    South,
    East,
    West,
}

//classic struct
struct User {
    active: bool,
    name: String,
    email: String,
}
fn main() {
    {
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
        let user2 = User {
            email: String::from("cooldude420@email.com"),
            ..user //another syntax to get the same results as update syntax from another instance
        };
        println!(
            "email is: {}, username: {}, user status is active:{}",
            user2.email, user2.name, user2.active
        );
        // println!(
        // "name is {},email is {} and the user is active:{}",
        // user.name, user.email, user.active
        // ); // this will result an error since the ownership of String is passed to user2 (remember heap and stack data)
    }
    {
        //tuple struct
        struct Color(u32, u32, u32);
        let black = Color(0, 0, 0);
        println!(
            "black 1: {},black 2: {},black 3: {}",
            black.0, black.1, black.2
        );
    }
    {
        //Unit struct
        struct AlwaysEqual;
        fn main() {
            let subject = AlwaysEqual;
        }
    }
    {
        let width = 30;
        let height = 40;
        println!(
            "The area of rectangle is {} square picels",
            area(width, height)
        );
        fn area(width: i32, height: i32) -> i32 {
            width * height
        }
    }
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
