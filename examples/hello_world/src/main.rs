proc_use::proc_use_inline!(mod_all
               {
                   #[disable]
                   use foo::*;
                   use bar::bar;
               }
);

fn main() {
    println!("hi");
}
