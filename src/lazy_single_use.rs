// Defines a number of self-rewriting macros, which expand to the body passed
// in when first called and then rewrite themselves to be no-ops on the second
// call.
macro_rules! lazy_single_use {
    ($($macname:ident => ($($body:tt)*);)+) => (
        $(
            macro_rules! $macname {
                () => (
                    $($body)*
                    macro_rules! $macname {
                        () => ();
                    }
                );
            }
        )+
    );
}

lazy_single_use! {
    print_zero => (println!("Zero!"););
    print_one => (println!("One!"););
    print_two => (println!("Two!"););
    print_three => (println!("Three!"););
}

fn main() {
    print_zero!();
    print_zero!();
    print_zero!();
    print_one!();
    print_one!();
    print_one!();
    print_two!();
    print_two!();
    print_two!();
    print_three!();
    print_three!();
    print_three!();
}

// Useful for macros which need to avoid errors from redundant use items/name
// collisions. Okay, honestly it's not all that useful, but it's still kinda
// cool.
//
// lazy_single_use! {
//     _use_Add => ( use std::ops::Add; );
//     _use_Sub => ( use std::ops::Sub; );
//     _use_Mul => ( use std::ops::Mul; );
//     _use_Div => ( use std::ops::Div; );
// }
//
// // Pretend this is macro generated
// mod __anonymous {
//     _use_Add!();
//     pub struct ThingWhichNeedsAdd {
//         // ...
//     }
//     // ... stuff here ...
//     // Another struct using Add gets auto-generated:
//     _use_Add!();
//     pub struct AnotherThingWhichNeedsAdd {
//         // ...
//     }
// }
// pub use __anonymous::*;
