fn main() {
    let user = User {
        username: String::from("nyanpasu"),
        email: String::from("nyanpasu@example.com"),
        id: 1,
        active: true
    };

    println!(
        "==User==\n\n  username: {},\n  email: {},\n  id: {},\n  active: {}",
        user.username,
        user.email,
        user.id,
        user.active
    );
}

struct User {
    username: String,
    email: String,
    id: u64,
    active: bool,
}