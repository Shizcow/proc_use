proc_use::proc_use_inline!(
    {
        #[mod_field]
        use foo::*;
        use bar::bar;
    }
);

fn main() {
    println!("hi");
}
