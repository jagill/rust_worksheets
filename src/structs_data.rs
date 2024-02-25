/**
 * Structs are one of the main types in Rust.  They are like classes, but:
 * 1. There is no subclassing/inheritance/etc (shared behavior comes from traits)
 * 2. There is no reflection/introspection
 * 3. They can be on the stack, and they can be _forced_ to be on the stack.
 **/

// Unit structs have no data, but can be quite useful!
// They are 0-sized: they take up 0 bytes, and in fact a Vec of many unit structs has 0 bytes of data on heap.
struct UnitFoo;

// Tuple structs take a tuple; this is really sugar for a normal struct with integer field names.
// So tuple_foo.0 is type bool, tuple_foo.2 is type String, etc.
// We already saw a 1-tuple struct in crate::ownership::Foo.
struct TupleFoo(bool, String);

// Most times you use a normal struct.  Access its members with foo.ok or foo.msg,
// but be careful with ownership!  You'll mess up soon enough, but the compiler's got your back.
struct Foo {
    ok: bool,
    msg: String,
}

#[cfg(test)]
mod tests {
    // Pro-tip: Using * in `use` statements is an antipattern for "imports".
    // But for inline modules (particularly tests), it reduces cognitive burden
    // to have "everything above" in scope with the same symbols.
    use super::*;

    #[test]
    fn test_structs_data_contruction() {
        let foo1 = Foo {
            ok: true,
            // Wait, didn't we use String::from before?  What's this to_owned?
            msg: "All's good".to_owned(),
        };
        // You can elide some or all field assignment if you use a correctly named variable.
        let ok = true;
        // Wait, didn't we just use `to_owned`?  What's the deal?
        let msg = "All's good".to_string();
        let foo2 = Foo { ok, msg };
        assert_eq!(foo1.msg, foo2.msg);
        // EXERCISE: Uh-oh, why can't we check for equality of Foo?  Search for PartialEq and get this to compile.
        // assert_eq!(foo1, foo2);
    }

    #[test]
    fn test_structs_data_member_access() {
        let foo = Foo {
            ok: true,
            msg: "All's good".to_owned(),
        };
        let is_ok = foo.ok;
        // EXTRA CREDIT: Why can't we use inline string interpolation like before?
        // (Hint: see https://doc.rust-lang.org/rust-by-example/hello/print.html)
        println!("foo.ok {}, is_ok {}", foo.ok, is_ok);

        let the_msg = foo.msg;
        // EXERCISE: Oh no! This won't compile.  Why not?
        // Find a way to make it compile.  Hint: make a copy...
        // println!("foo.msg {}, the_msg {}", foo.msg, the_msg);
    }

    #[test]
    fn test_structs_data_destructuring() {
        // To move _all_ of the member variables, we can use destructuring.
        // This moves all the members into new variables.
        let foo = Foo {
            ok: true,
            msg: "All's good".to_owned(),
        };

        // Destructuring uses the same elision as construction
        let Foo { ok: is_ok, msg } = foo;

        println!("is_ok {is_ok}, msg {msg}");
        // Extra Credit: Learn how to use `..` and `_` to only keep some destructured members
    }

    #[test]
    fn exercise_structs_data_1() {
        let foo = TupleFoo(true, "cat".to_owned());

        unimplemented!(
            r#"
            How do we deconstruct a tuple struct?
            Assign the components by ownership to vars ok and cat.
            Uncomment the lines and make them compile and pass.
        "#
        );

        // assert_eq!(ok, true);
        // assert_eq!(cat, "cat");
    }

    #[test]
    fn exercise_structs_data_2() {
        let foo = TupleFoo(true, "cat".to_owned());

        unimplemented!(
            r#"
            Make fn that takes foo by ownership and just returns the String member.
            Assign it to a var msg and make the below compile.
            "#
        );

        // assert_eq!(msg, "cat");
    }

    #[test]
    fn exercise_structs_data_3() {
        let foo = TupleFoo(true, "cat".to_owned());

        unimplemented!(
            r#"
            Make fn that takes foo by _reference_ and returns a copy of the String member.
            Assign it to a var msg and make the below compile.
            "#
        );

        // assert_eq!(msg, foo.msg);
    }
}
