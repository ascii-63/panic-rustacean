#[derive(Debug)]
struct Reactangle {
    height: u32,
    width: u32,
}

fn area(reactangle: &Reactangle) -> u32 {
    reactangle.width * reactangle.height
}

fn main() {
    let reac = Reactangle {
        width: 30,
        height: 50,
    };

    println!("{}", area(&reac)); // -> stdout

    println!("{:?}", reac); // Simple debug
    println!("{:#?}", reac); // Prettier debug

    dbg!(&reac); // -> stderr
}
