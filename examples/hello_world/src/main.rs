proc_use::proc_use_inline! {
    #[__mod]
    use foo::*;
    #[__mod]
    use bar::bar;
}

fn main() {
    println!("hi");
    foo();
    bar();
}
