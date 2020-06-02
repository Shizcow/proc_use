mod foo; // TODO proc_mod

proc_use::proc_use! {
    ["foo"]
}

fn main() {
    foo();
}
