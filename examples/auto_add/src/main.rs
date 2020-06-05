proc_use::proc_use! {
    // mod("../external/foo.rs");
    const r#mod: _ = "../external/foo.rs";
}


fn main() {
    foo::foo();
}
