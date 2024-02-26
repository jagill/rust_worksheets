// At some point using Rust, you _will_ struggle with lifetimes.
// As it is a core underlying mechanism of Rust's ownership system,
// it is one of the big conceptual differences in Rust and the
// key to mastering ownership and borrowing.
//
// Historically, lifetimes were supposed to be a compiler detail
// that "normal" people never interacted with.  And most of the
// time this is true.  Getting into areas where you need to deal
// with lifetimes is Rust on "hard mode".  To make it trickier,
// certain constructs (like async/await) use lifetimes in an
// implicit manner that makes debugging harder.
//
// An open secret is that every borrow (& or &mut) has a hidden
// lifetime.  One mixed blessing is that Rust elides lifetimes when it can
// figure them out, which means you generally don't see them.
// The flip side is that when you do see them, it's the hard
// cases that the compiler can't figure out, and you haven't built
// up the mental muscles on all the easy cases.
//
// In this worksheet we'll work through lifetimes explicitly,
// showing when they are elided and the rules to elide them.
// And in the future, when you are frustrated with lifetimes,
// remember one very important thing: the compiler lifetime
// error is most likely protecting you from a very subtle
// bug, involving non-determinacy, race-conditions, or multi-
// threaded data access.

#[cfg(test)]
mod tests {
    // FOUNDATIONAL PRINCIPAL: Lifetimes do _not_ affect runtime behavior; they
    // are only there for compile-time borrow checking.  If you get the lifetimes
    // to compile, you are _safe_!

    const NUMBER: i32 = 1;

    #[test]
    fn test_lifetimes_static() {
        // Easy case: 'static
        // 'static lifetime means it lives the length of the program.  It will
        // always be available.  It's mostly from literals and constants, but it crops up in
        // other cases too.
        let literal_str: &'static str = "data";
        let num_ref: &'static i32 = &NUMBER;

        // You generally cannot make a 'static ref to a non-literal/non-const, because the
        // object doesn't last as long as the program.
        let scoped_num: i32 = 2;
        // Doesn't compile!
        // let scoped_num_ref: &'static i32 = &scoped_num;

        // Extra-credit: look up std::mem::leak
    }

    #[test]
    fn test_lifetimes_function_signatures() {
        // Non-static lifetimes are generics in functions/types/impls.
        fn foo1<'a>(s: &'a str) -> usize {
            s.len()
        }

        // This 'a has nothing to do with the last 'a
        struct Bar<'a> {
            _s: &'a str,
        }

        impl<'a> Bar<'a> {
            pub fn s(&self) -> &'a str {
                self._s
            }
        }

        // Similar to type generics, the compiler solves lifetimes based on function calls.
        // QUIZ: In the next call to foo1, what does 'a resolve to?
        let literal_str: &'static str = "data";
        assert_eq!(foo1(literal_str), 4);

        // You can give each input parameter a different lifetime
        fn foo2<'a, 'b>(s1: &'a str, s2: &'b str) -> usize {
            s1.len() + s2.len()
        }

        // You can force the constraints to be bound by using one lifetime (like generics!)
        // In this case, it doesn't matter, but in some cases it does.
        fn foo2_2<'a>(s1: &'a str, s2: &'a str) -> usize {
            s1.len() + s2.len()
        }

        // Any lifetime in the return type must be related to in the input type.
        fn first_str<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str {
            s1
        }
    }

    #[test]
    fn test_lifetimes_elision() {
        // Elision 1: If there is no lifetime in the return value, the input lifetimes _do not matter_.
        // Because of this, the compiler trivially solves this and elides them.
        fn bar<'a, 'b>(s1: &'a str, s2: &'b str) -> usize {
            s1.len() + s2.len()
        }
        fn bar_1(s1: &str, s2: &str) -> usize {
            s1.len() + s2.len()
        }

        // Elision 2: If there is only one input and output lifetime, they are viewed as the same.
        fn baz<'a>(s1: &'a str) -> &'a str {
            &s1[..4]
        }
        fn baz_1(s1: &str) -> &str {
            &s1[..4]
        }

        // Elision 3: Any input lifetime not affecting the output lifetime can be omitted or replaced with '_.
        fn first_str<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str {
            s1
        }
        fn first_str_1<'a>(s1: &'a str, s2: &str) -> &'a str {
            s1
        }
        // Note that you cannot elide 'a because the compiler needs to know how the output is related to the input.

        // Elision 4: If &self is in the parameters, the output lifetime is assumed to be the same as &self.
        struct Foo(String);
        impl Foo {
            pub fn frotz<'a, 'b>(&'a self, other: &'b str) -> &'a str {
                &self.0[..1]
            }
            pub fn frotz_1(&self, other: &str) -> &str {
                &self.0[..1]
            }

            pub fn fribble<'a, 'b>(&'a self, other: &'b str) -> &'b str {
                &other[..1]
            }
            pub fn fribble_1<'b>(&self, other: &'b str) -> &'b str {
                &other[..1]
            }
        }
    }
}
