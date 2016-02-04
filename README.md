# macro-abuse
A few examples of Rust macro abuse which I found particularly fun.

is_eq! demonstrates a method of comparing lists of token trees at compile-time.

lazy_single_use! and lazy_cycle_instantiate! demonstrate how macro_rules! can be used to achieve a form of mutable state, retaining information between macro calls by rewriting the called macro. Has a few problems, most notably that as a result the called macro cannot be used in expression positions.
