fn main() {
    let s1 = String::from("HELL");
    let len_s1 = calc_len(&s1);
    println!("Len of {s1}: {len_s1}");

    let ref_to_nothing = dangling_ptr();
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn change(s: &String) {
    s.push_str("string"); // s is a `&` reference, so the data it refers to cannot be borrowed as mutable
}

fn change_(s: &mut String){
    s.push_str("string");
}

/// Dangle returns a reference to a String
fn dangling_ptr() -> &String{
    let s = String::from("PTR");
    &s
}

fn no_dangle() -> String {
    let s = String::from("HELL");
    s
}