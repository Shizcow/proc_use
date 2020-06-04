<<<<<<< HEAD
mod foo; // TODO proc_mod
mod bar;

proc_use::proc_use! {
    #![mod]
    use foo::*;
=======
proc_use::proc_use_inline! {
    #[mod]
    use foo::*;
    #[mod]
>>>>>>> custom_syntax
    use bar::bar;
}

fn main() {
<<<<<<< HEAD
    //foo();
    //bar();
=======
    println!("hi");
    foo();
    bar();
>>>>>>> custom_syntax
}
