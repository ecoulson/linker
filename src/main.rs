fn main() {
    let args = std::env::args().skip(1);

    for filename in args {
        dbg!(filename);
    }

    println!("Hello, world!");
}
