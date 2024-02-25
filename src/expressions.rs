// Pro-tip: Items marked with cfg(test) are only included in tests;
// run them with `buck test`
#[cfg(test)]
mod tests {
    // mod tests {} is a convention to put your inline unit tests.
    // It could be anything, but conventions are nice.

    // This marks tests to be run
    #[test]
    fn test_expressions_basic_assignment() {
        let x: i32 = 1;
        println!("{x}");
        assert_eq!(x, 1);

        // Rust is pretty good at type inference, although there are many types of integers.
        let y = 2;
        assert_eq!(y, 2);
    }

    #[test]
    fn test_expressions_mutable_assignment() {
        // `let` makes an immutable binding, `let mut` allows the variable to change.
        let mut x = 1;
        x += 1;
        assert_eq!(x, 2);
    }

    #[test]
    fn test_expressions_delayed_assignment() {
        // You can declare a variable without initialization.
        let x: i32;
        // ... but you can't use it until you do.
        //XX println!("{x}");
        x = 1;
        assert_eq!(x, 1);
    }

    #[test]
    fn test_expressions_complex_assignment() {
        // In rust, almost everything is an expression.  This is very powerful!  For example,
        // `if` expressions can remove the need for delayed assignment.
        let x = if true { 1 } else { 2 };
        assert_eq!(x, 1);

        // Also, blocks are expressions!  They evaluate to the last expression.
        let y = {
            let z = 1;
            z + 1
        };
        assert_eq!(y, 2);
        // Variables are only valid in their block, so z is no longer in scope.
        // This allows more complex initialization without leaking the intermediate values.
        // EXERCISE: Uncomment this and try to compile.
        // println!("{z}");

        // loop makes an infinite loop.  But breaking can return the value!
        // This is useful if you want to assign a value based on a condition you need to iterate on.
        let mut a = 0;
        let b = loop {
            a += 1;
            if a == 3 {
                break a;
            }
        };
        assert_eq!(b, 3);
    }

    #[test]
    fn exercise_expressions_shadowing() {
        let x = 1;
        {
            let x = 2;
            assert_eq!(x, 2);
        }
        unimplemented!(
            r"
            EXERCISE: What goes in this assert?
            Correct and uncomment the following assert.
        "
        );
        // assert_eq!(x, ???);
    }

    #[test]
    fn exercise_expressions_semicolon() {
        let x = {
            1;
        };
        unimplemented!(
            r"
            EXERCISE: What type is x?  What value is x?
            You'll probably need to read about Rust types.
            Correct and uncomment the following assert.
            Hint: https://fburl.com/4g00abut
        "
        );
        // assert_eq!(x, ???);
    }
}
