// Compares two token tree lists. Expands to the token tree list inside the first block if the
// lists are equivalent, and the second if they are not. Expensive - generates a new macro for
// every invocation! This macro is only valid in item position, due to its use of macro_rules.
macro_rules! is_eq {
    (
        if ($($thingA:tt)*) == ($($thingB:tt)*) {
            $($if_true:tt)*
        } else {
            $($if_false:tt)*
        }
    ) => (
        macro_rules! is_eq_test {
            ($($thingA)*, $($thingA)*) => ($($if_true)*);
            ($($thingA)*, $($thingB)*) => ($($if_false)*);
        }

        is_eq_test!($($thingA)*, $($thingB)*);
    );
}

is_eq! {
    if (a bunch of tokens!) == (another bunch of tokens!) {
        fn eq_test_1() { println!("They're equal!"); }
    } else {
        fn eq_test_1() { println!("As expected, they're not equal."); }
    }
}

is_eq! {
    if (a pile of equivalent tokens.) == (a pile of equivalent tokens.) {
        fn eq_test_2() { println!("As expected, they're equal."); }
    } else {
        fn eq_test_2() { println!("They're not equal!"); }
    }
}

fn main() {
    eq_test_1();
    eq_test_2();
}
