// TODO: Fix the compiler error without taking the macro definition out of this
// module.
mod macros {
    #[macro_export] // This makes it available at the crate root
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
