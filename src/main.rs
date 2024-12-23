fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        panic!("Failed to get appropriate arguments for `cargo-wiki`. Make sure you have installed it correctly.")
    }
    args.drain(0..2);

    for (i, arg) in args.iter().enumerate() {
        println!("{} --{}", i, arg)
    }
}
