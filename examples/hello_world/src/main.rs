proc_use::proc_use! {
    #[mod]
    use foo::*;
    #[mod("../external/bar.rs")]
    use bar::bar;
}

fn main() {
    println!("hi");
    foo();
    bar();
}
