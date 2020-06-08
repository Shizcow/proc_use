include!(concat!(env!("OUT_DIR"), "/proc_use.rs"));

fn main() {
    aliased::foo();
}
