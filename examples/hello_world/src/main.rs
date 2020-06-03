mod foo; // TODO proc_mod
mod bar;

proc_use::proc_use! {
    #![mod]
    use foo::*;
    use bar::bar;
}

fn main() {
    //foo();
    //bar();
}
