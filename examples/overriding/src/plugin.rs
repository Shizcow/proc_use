use overrider::*;

#[override_default]
pub fn foo() {
    println!("Overriden foo");
}
