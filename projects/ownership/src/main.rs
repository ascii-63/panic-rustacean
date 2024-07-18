fn main() {
    // STACK
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // HEAP
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // MOVE
    {
        let s1 = String::from("hremions");
        let s2 = s1;

        // println!("{s1}"); //error[E0382]: borrow of moved value: `s1`
    } // Free only s2, done
}

////////////////////////////////////////////

fn ownership_and_functions() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

////////////////////////////////////////////

fn return_values_and_scope() {
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    let s2 = String::from("Hell no"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

/// This function will move its return value into the function that calls it
fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

/// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope

    a_string // a_string is returned and moves out to the calling function
}

////////////////////////////////////////////

fn return_multiple_values_using_tuple() {
    let s1 = String::from("Fksit");
    let (s2, len) = calc(s1);
}

fn calc(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
