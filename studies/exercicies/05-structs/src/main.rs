struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
// struct UserRef {
//     username: &str, //Reference need lifetime
//     email: &str,    //Reference
//     sign_in_count: u64,
//     active: bool,
// }

struct Point(i32, i32, i32); // Struct Tuple
struct Unit {} // UnitLike struct vazio
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User {
        email: String::from("alguem@exemplo.com"),
        username: String::from("algumnome123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user! {}", user1.email);

    let user2 = build_user(
        String::from("alguem@exemplo.com2"),
        String::from("algumnome1232@exemplo.com"),
    );
    println!("user! {}", user2.email);

    let user3 = User {
        email: String::from("algu2em@exemplo.com"),
        ..user1 // Struct update syntax -> equivalent a spread in JS
    };
    println!("user3! {}", user3.email);

    let origin = Point(5, 0, 0);
    println!("origin! {}", origin.0);
}
