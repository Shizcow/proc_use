proc_use::proc_use_inline! {
    #[mod]
    use foo::*;
    #[mod]
    use bar::bar;
}

fn main() {
    println!("hi");
    foo();
    bar();
}
