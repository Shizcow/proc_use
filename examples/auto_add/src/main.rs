proc_use::proc_use! {
    mod("../external/foo.rs");
}

fn main() {
    foo::foo();
}
