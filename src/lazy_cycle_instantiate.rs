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

macro_rules! lazy_cycle_instantiate {
    (@remove
        ($($remove:tt)*) $name:ident
        { $($searched:tt)* }
        { ($($input:tt)*) => { $($body:tt)* } $($unsearched:tt)* }
    ) => (
        is_eq! {
            if ($($remove)*) == ($($input)*) {
                lazy_cycle_instantiate! {
                    $name:
                    $($searched)*
                    $($unsearched)*
                    ($($input)*) => { $($body)* }
                }
            } else {
                lazy_cycle_instantiate! {
                    @remove
                    ($($remove)*)
                    $name
                    { $($searched)* ($($input)*) => { $($body)* } }
                    { $($unsearched)* }
                }
            }
        }
    );
    (@recurse $all:tt { $($build:tt)* } $name:ident ( $($input:tt)* ) => { $($body:tt)* } $($more:tt)*) => (
        lazy_cycle_instantiate! {
            @recurse
            $all
            {
                $($build)*
                ($($input)*) => (
                    $($body)*
                    lazy_cycle_instantiate!(@remove ($($input)*) $name {} $all);
                );
            }
            $name
            $($more)*
        }
    );
    (@recurse $all:tt { $($build:tt)* } $name:ident) => (
        macro_rules! $name {
            $($build)*
        }
    );
    ($name:ident: $($more:tt)*) => (
        lazy_cycle_instantiate!(@recurse { $($more)* } {} $name $($more)*);
    );
}

lazy_cycle_instantiate! {
    print_number_sequence:
    () => { println!("0"); }
    () => { println!("1"); }
    () => { println!("2"); }
    () => { println!("3"); }
}

fn main() {
    print_number_sequence!(); // 0
    print_number_sequence!(); // 1
    print_number_sequence!(); // 2
    print_number_sequence!(); // 3
    print_number_sequence!(); // 0
    print_number_sequence!(); // 1
    print_number_sequence!(); // 2
    print_number_sequence!(); // 3
    print_number_sequence!(); // 0
    print_number_sequence!(); // 1
}
