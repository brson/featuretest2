pub fn run() {
    println!("mydep");
    let mut have = false;
    if cfg!(feature = "default") {
        println!("default");
        have = true;
    }
    if cfg!(feature = "foo") {
        println!("foo");
        have = true;
    }
    if cfg!(feature = "bar") {
        println!("bar");
        have = true;
    }
    if !have {
        println!("no features");
    }
    println!();
    mydep2::run();
}
