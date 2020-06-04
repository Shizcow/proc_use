proc_use::proc_use_inline! {
    //#[mod]
    //use foo::*;
    #[__mod("../external/bar.rs")] // TODO sanitize
    use bar::bar;
}

fn main() {
    println!("hi");
    //foo();
    //bar();
}
