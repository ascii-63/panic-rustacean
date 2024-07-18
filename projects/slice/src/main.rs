use std::io::Bytes;

fn the_problem() {
    let mut s = String::from("HELL NO");
    let word = first_word_index(&s);

    s.clear();
    // word still has the value 4 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

//////////////////////////////////////////

fn main() {
    let s = String::from("HEKK NO");

    let mut hekk = &s[0..4];
    let mut no = &s[5..7];

    // SAME AS
    hekk = &s[..4];
    no = &s[5..];

    let mut word = first_word(&s);
    println!("{word}");

    let a_s = "HECK NO";

    word = first_word(&a_s);
    // SAME AS
    word = first_word(a_s);
    println!("{word}");

}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }

    &s[..]
}