struct User {
    active: bool,
    name: String,
    email: String,
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
}
