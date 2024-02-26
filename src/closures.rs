/// Rust's ownership and lifetime rules make closures safer, but require you
/// to think about the values that the closure closes over.  Since they may
/// be mutated or consumed, it may be impossible to call the closure more
/// than once, or to have the closure be used by multiple owners.

// Full closure signature:
// |p1: type1, p2: type2| -> ret_type { body }
// But we almost always elide things.
// |p1: type1, p2: type2|             { body }
// |p1       , p2       |             { body }
// If body is a single expression
// |p1       , p2       |               expr
// Rust's inference is pretty good, but sometimes you'll need to put in type information.

#[cfg(test)]
mod tests {
    #[test]
    fn test_closures_fn() {
        // It's easiest if the closure doesn't capture anything
        let f1 = || 1;
        assert_eq!(f1(), 1);
        assert_eq!(f1(), 1);

        // A close second: It captures something that it doesn't mutate.
        let x = 2;
        let f2 = || x.clone();
        assert_eq!(f2(), 2);
        assert_eq!(f2(), 2);

        // The trait that these satisfy is called `Fn`, written as:
        // Fn(i32, i32) -> i32
    }

    #[test]
    fn exercise_closures_fn_mut() {
        // What if you want to capture mutable state?
        let mut x = 0;
        let f3 = |y: i32| {
            x += 1;
            x + y
        };

        // The trait that these satisfy is called `FnMut`, written as:
        // FnMut(i32, i32) -> i32

        unimplemented!(
            r#"
            These asserts DO NOT COMPILE!  Why?
            Because f3 mutates x, it is mutating its captured state.  Really, a
            closure is struct with a `call` method.  So it must _itself_ be
            declared mutable.  Make this work.
            "#
        );

        // assert_eq!(f3(5), ???);
        // assert_eq!(f3(5), ???);
    }

    #[test]
    fn exercise_closures_fn_once() {
        // If the closure uses its state by _move_, it can only be used once.
        let s = String::from("just me");
        let f4 = || s;

        // The trait that this satisfies is called `FnOnce`, written as:
        // FnOnce(i32, i32) -> i32

        unimplemented!(
            r#"
            How many asserts can you do by calling f4?
            "#
        );
    }
}
