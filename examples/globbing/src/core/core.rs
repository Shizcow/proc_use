include!(concat!(env!("OUT_DIR"), "/proc_use.rs"));

pub fn execute() {
    foo();
    bar();
    baz();
}
