proc_use::proc_use_inline! {
    #[__mod]
    use foo::*;
    use bar::bar;
}

fn main() {
    println!("hi");
}
