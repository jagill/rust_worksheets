/*
In Rust, there are two fundamental ways to handle errors.

The first is `panic!()` and it's sugary friends (`assert!`, `unimplemented!`, `unreachable!`,
`todo!`, etc).  Panics will abort the process, making a best-effort attempt to
unwind the stack, call destructors, etc.  Panic/assert should only be used for
cases where no forward progress is possible.  For example, assert is typically used
when an invariant/contract can't be expressed in the type system, but must be
enforced.  In an argument to a function cannot be 0, the function documentation
should indicate this, and an `assert_ne!(x, 0)` placed at the top of the function
body.  Similarly, if a function return has an invariant, the caller can introduce
an assert for safety.

The second way to handle errors is with the Result<T, E> type, which has two variants:
Ok(T) for the "good" case, and Err(E) for the "bad" case.
Error handling with Result is explicit and required.  There is no equiavlent of
unchecked Exceptions in Java, or Exceptions in general.  Instead, a `Result`
enum is used.  Similar to the Option-type, there's a variety of syntax to make
working with them quite ergonomic.

Helpfully, if you join on a thread that panics, the join method returns
a Result::Err.  Panics can also be handled across FFI boundaries, but that's
beyond the scope of this module.
*/

#[cfg(test)]
mod tests {
    #[test]
    fn test_errors_basic() {
        struct Foo(i32);
        // Result<T, E> has two variants: Ok(T) for the "good" type, and
        // Err(E) for the error type.  The Error type can be anything.  We'll
        // talk about best practices later, but for now let's use String to
        // return an error message.
        let foo_ok: Result<Foo, String> = Ok(Foo(1));
        let foo = match foo_ok {
            Ok(f) => f,
            Err(e) => {
                println!("Returning a default because of error: {e}");
                Foo(0)
            }
        };
        assert_eq!(foo.0, 1);

        let foo_err: Result<Foo, String> = Err("Something went wrong...".to_owned());
        let foo2 = match foo_err {
            Ok(f) => f,
            Err(e) => {
                println!("Returning a default because of error: {e}");
                Foo(0)
            }
        };
        assert_eq!(foo2.0, 0);
    }

    #[test]
    fn test_errors_short_circuit() {
        // Like Option, the `?` operator helps prevent error handling from being too
        // verbose.  It's sort of like a `throw` statement in other languages, allowing
        // you to bubble the error up to the caller.

        fn maybe_cast(i_res: Result<i32, String>) -> Result<f32, String> {
            let i: i32 = i_res?;
            Ok(i as f32)
        }

        assert_eq!(maybe_cast(Ok(1)), Ok(1.0));
        assert_eq!(maybe_cast(Err("Bad".to_owned())), Err("Bad".to_owned()));
    }

    #[test]
    fn test_errors_forced_handling() {
        // Since Result is part of type type system, you _must_ handle it.
        // You cannot inadvertantly forget and let it bubble to the top.

        // Like Option, the easiest and worst way to handle a Result is with `.unwrap()` or `.expect()`.
        // These will give you the contents of Ok, but will _panic_ on Err.
        // If you are writing a library, this is probably a bad idea: You are
        // deciding for the binary/service that it cannot continue, instead of
        // propagating the error and letting them decide.

        // Instead, use `?` (particularly with `From`, coming soon), or match/if-let.
        let x: i32 = Result::<i32, String>::Ok(1).expect("I didn't think this could fail...");
        assert_eq!(x, 1);

        fn safe_div(numer: i32, denom: i32) -> Result<i32, String> {
            if denom != 0 {
                Ok(numer / denom)
            } else {
                Err("div by zero".to_owned())
            }
        }

        let quotient = match safe_div(12, 0) {
            Ok(q) => q,
            Err(e) => {
                println!("Had an error: {e}");
                0
            }
        };
        assert_eq!(quotient, 0);
    }

    #[test]
    fn exercise_errors_combinators() {
        unimplemented!(
            r"
        While `match` statements are all you technically need to work with Options,
        they can quickly become deeply nested and hard to read.  Instead, combinators
        allow easy manipulation of Options.

        For each of these, uncomment and correct the asserts.  You may find the
        documentation for Result helpful: XXXX
        "
        );
        // Type alias to make things less verbose.
        type Res = Result<i32, &'static str>;

        //assert_eq!(Res::Ok(1).map(|x| x + 1), ???)
        //assert_eq!(Res::Err("bad").map(|x| x + 1), ???)

        //assert_eq!(Res::Ok(1).map_err(|s| s.to_uppercase()), ???)
        //assert_eq!(Res::Err("bad").map_err(|s| s.to_uppercase()), ???)

        //assert_eq!(Res::Ok(1).unwrap_or(2), ???)
        //assert_eq!(Res::Err("bad").unwrap_or(2), ???)

        //assert_eq!(Res::Ok(1).or(Res::Ok(2)), ???)
        //assert_eq!(Res::Err("bad").or(Res::Ok(2)), ???)

        //assert_eq!(Res::Ok(1).and(Res::Ok(2)), ???)
        //assert_eq!(Res::Err.and(Res::Ok(2)), ???)

        //assert_eq!(Res::Ok(1).unwrap_or_else(|x| x + 1), ???)
        //assert_eq!(Res::Err.unwrap_or_else(|x| x + 1), ???)

        //assert_eq!(Res::Ok(1).unwrap_or_default(), ???)
        //assert_eq!(Res::Err.unwrap_or_default(), ???)

        //assert_eq!(Res::Ok(1).or_else(|| Res::Ok(2)))
        //assert_eq!(Res::Err.or_else(|| Res::Ok(2)))

        //assert_eq!(Res::Ok(1).and_then(|x| Res::Ok(x+1)), ???)
        //assert_eq!(Res::Err.and_then(|x| Res::Ok(x+1)), ???)

        //assert_eq!(Res::Ok(1).ok(), ???);
    }

    #[test]
    fn exercise_errors_map_err_short_circuit() {
        unimplemented!(
            r"
        With map_err and `?`, you can short-curcuit an Err in a function that has a different
        error type.

        Uncomment foo and make it compile with map_err and `?`, and have the assert pass.
        "
        );

        fn read_stuff() -> Result<[u8; 10], std::io::Error> {
            use std::io::prelude::*;

            let mut f = std::fs::File::open("/dev/random")?;
            let mut buffer = [0; 10];
            // read up to 10 bytes
            let n = f.read(&mut buffer)?;
            Ok(buffer)
        }

        // fn foo() -> Result<usize, String> {
        //     let buffer = read_stuff().????;
        //     buffer.len()
        // }
        //
        // assert_eq!(foo(), Ok(10));
    }

    #[test]
    fn exercise_errors_structured_errors() {
        enum QueryError {
            /// May be transient; retry soon.
            Network { http_code: u16, reason: String },
            /// User error; don't retry without changing input
            Parsing { row: u32, col: u32 },
            /// Non-transient server error; file a bug with oncall
            Server { desc: String },
        }

        fn run_query(query: &str) -> Result<String, QueryError> {
            Err(QueryError::Parsing { row: 10, col: 4 })
        }

        unimplemented!(
            r#"
            Rust allows structured errors so that callers can respond intelligently to the type of error.
            `match` on the output of run_query, giving a different user message based on whether it's a
            success, or what find of failure it is.
            "#
        );
    }

    #[test]
    fn exercise_errors_thiserror() {
        unimplemented!(
            r#"
            Rust actually defines a trait `std::error::Error`, which gives functions commonly
            needed for reporting, like a user-facing message and the ability to nest errors.
            It's best to implement this, so that your error is interoperable with other errors.

            dtolnay has made a great library called `thiserror`, which does most of the work
            for you.  Check out https://docs.rs/thiserror/latest/thiserror/ and rewrite
            QueryError above using `thiserror`.
            "#
        );
    }

    #[test]
    fn exercise_errors_anyhow() {
        unimplemented!(
            r#"
            dtolnay has made another error library `anyhow`, for when you just want throw
            undifferentiated errors.  This is useful in a CLI or executable when you can't
            reasonable recover from the error.  It has nice features like ease-of-use and
            ergonomic adding of context.  It also can "absorb" anything that implements
            std::error::Error (technically implements `From`), so it can easily

            PLEASE NOTE: It is bad practice to use this in libraries
            that others might consume, because it doesn't give enough information for them
            to intelligently respond.  It's equivalent to a Java unchecked RuntimeException.
            Only use it for executables that you control.
            "#
        );

        #[derive(thiserror::Error, Debug)]
        enum InnerError {
            #[error("Uh-oh, something is WRONG {0}")]
            Wrong(String),
        }

        fn inner() -> Result<i32, InnerError> {
            let some_call = Err(InnerError::Wrong(String::from("sad")));
            // Aside: Check out this sweet type inference!
            let something_else = some_call?;
            Ok(something_else)
        }

        fn outer() -> Result<i32, anyhow::Error> {
            // anyhow automatically converts from InnerError!
            let i = inner()?;
            if i == 5 {
                // quickly make an anyhow::Error with `bail!`.
                anyhow::bail!("I don't like 5!");
            }

            Ok(i)
        }

        match outer() {
            Ok(i) => println!("Got {i}"),
            Err(e) => unimplemented!("Fill this in!"),
        }
    }
}
