/*
 * Variables in Rust can never be null.  There are no null pointers.
 * To express the possibility of absence, use Option.  Its definition:
 * enum Option<T> { None, Some(T) }
 * (Don't worry about the <T>, that's for generics, which we'll get to later.)
 *
 * Encoding nullity into the type system means many runtime errors (like NPEs)
 * are moved to compile-time, eliminating a whole class of reliability issues.
 * While everything can be done with the standard enum match primitive, there
 * are a variety of methods that make working with Options very safe and
 * ergonomic.
 */

#[cfg(test)]
mod tests {
    #[test]
    fn test_options_basic() {
        struct Foo(i32);
        let some_foo: Option<Foo> = Some(Foo(1));
        let foo = match some_foo {
            Some(f) => f,
            None => Foo(-1),
        };
        assert_eq!(foo.0, 1);

        // We'll need a type hint here, since None can be a member of many different Options.
        let none_foo: Option<Foo> = None;
        let foo2 = match none_foo {
            Some(f) => f,
            None => Foo(-1),
        };
        assert_eq!(foo2.0, -1);
    }

    #[test]
    fn test_options_short_circuit() {
        // To prevent handling options from being too verbose, Rust has the `?` operator,
        // which early-returns None in functions that return Option.

        fn maybe_cast(i_opt: Option<i32>) -> Option<f32> {
            let i: i32 = i_opt?;
            Some(i as f32)
        }

        assert_eq!(maybe_cast(Some(1)), Some(1.0));
        assert_eq!(maybe_cast(None), None);
    }

    #[test]
    fn test_options_forced_handling() {
        // Since Option is part of type type system, you _must_ handle them.
        // You cannot inadvertantly forget and get a NullPointerException (or similar).

        // The easiest and worst way to handle them is with `.unwrap()` or `.expect()`.
        // These will give you the contents of Some, but will _panic_ on None.
        // Only use if you know something that the type system can't express.
        // Prefer match/if-let to unwrap.
        //
        // unwrap gives a generic message, but expect is better because you can put context
        // to help you debug if it gets triggered.
        let x: i32 = Some(1).expect("I really thought there was something here.");
        assert_eq!(x, 1);

        // But generally you _actually_ handle the None case, because it's important.
        let index = "the cat in the hat".find("cat");
        match index {
            Some(idx) => println!("Found a cat at index {idx}"),
            None => println!("Couldn't find the cat :("),
        }
    }

    #[test]
    fn exercise_options_combinators() {
        unimplemented!(
            r"
        While `match` statements are all you technically need to work with Options,
        they can quickly become deeply nested and hard to read.  Instead, combinators
        allow easy manipulation of Options.

        For each of these, uncomment and correct the asserts.  You may find the
        documentation for Option helpful: XXXX
        "
        );

        //assert_eq!(Some(1).map(|x| x + 1), ???)
        //assert_eq!(None.map(|x| x + 1), ???)

        //assert_eq!(Some(1).unwrap_or(2), ???)
        //assert_eq!(None.unwrap_or(2), ???)

        //assert_eq!(Some(1).or(Some(2)), ???)
        //assert_eq!(None.or(Some(2)), ???)

        //assert_eq!(Some(1).and(Some(2)), ???)
        //assert_eq!(None.and(Some(2)), ???)

        //assert_eq!(Some(1).unwrap_or_else(|x| x + 1), ???)
        //assert_eq!(None.unwrap_or_else(|x| x + 1), ???)

        //assert_eq!(Some(1).unwrap_or_default(), ???)
        //assert_eq!(None.unwrap_or_default(), ???)

        //assert_eq!(Some(1).or_else(|| Some(2)), ???)
        //assert_eq!(None.or_else(|| Some(2)), ???)

        //assert_eq!(Some(1).and_then(|x| Some(x+1)), ???)
        //assert_eq!(None.and_then(|x| Some(x+1)), ???)

        let x = Some(1);
        let y = x.take();
        //assert_eq!(x, ???);
        //assert_eq!(y, ???);
    }

    #[test]
    fn exercise_options_if_let() {
        unimplemented!(
            r"
        It's common to want to check if an option is Some, then unwrap it.  Don't
        do this!  It is more fragile and allows runtime errors -- your logic may
        be wrong, or someone later will change your code in such a way that it
        breaks.

        Instead, use the type system to help you.  Use if-let to rewrite this
        block to be runtime-safe.
        "
        );

        let x_opt = Some(1);
        if x_opt.is_some() {
            let x = x_opt.unwrap();
            assert_eq!(x, 1);
        }
    }
}
