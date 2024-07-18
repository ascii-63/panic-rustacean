struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        sign_in_count: 1,
        username: String::from("value"),
        email: String::from("@gmaill.com"),
    };

    user1.email = String::from("example@gmail.com");

    let mut user2 = User {
        username: String::from("hekk"),
        email: String::from("value"),
        ..user1 // the remaining fields not explicitly set should have the same value as the fields in the user1
    };

    /////////////////////

    let black = RGB(0, 0, 0);
    let origin = Point3D(0.0, 0.0, 0.0);

    ////////////////////

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}

/////////////////////////////////

struct RGB(i32, i32, i32);
struct Point3D(f64, f64, f64);

/////////////////////////////////

// Unit-Like Structs
struct AlwaysEqual;
