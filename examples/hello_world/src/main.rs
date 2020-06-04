proc_use::proc_use_inline! {
    #[mod]
    use {foo::*, bar::bar};
}

fn main() {
    println!("hi");
    foo();
    bar();
}
