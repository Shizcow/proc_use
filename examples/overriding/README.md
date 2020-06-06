# overriding
This example shows how `proc_use` can be used with the `overrider` crate, making it
significantly more powerful. Changing the boolean value of `plugin` in `build.rs`
modifies the import stack and overrides `foo`, completly changing the function of the
program. See (`overrider`)[https://github.com/Shizcow/overrider-rs] for more details.
