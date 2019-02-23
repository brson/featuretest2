pub fn run() {
    println!("mydep2");
    let mut have = false;
    if cfg!(feature = "default") {
        println!("default");
        have = true;
    }
    if cfg!(feature = "baz") {
        println!("baz");
        have = true;
    }
    if cfg!(feature = "qux") {
        println!("qux");
        have = true;
    }
    if !have {
        println!("no features");
    }
    println!();
}
