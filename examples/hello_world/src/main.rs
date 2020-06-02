mod foo; // TODO proc_mod
mod bar;

proc_use::proc_use! {
    ["foo::*", "bar::bar"]
}

fn main() {
    foo();
    bar();
}
