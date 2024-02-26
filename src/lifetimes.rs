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

    #[test]
    fn test_lifetimes_baby_steps() {
        // FOUNDATIONAL PRINCIPAL: Lifetimes do _not_ affect runtime behavior; they
        // are only there for compile-time borrow checking.  If you get the lifetimes
        // to compile, you are _safe_!
        //
        // Rule 0: 'static is easy.
        // 'static lifetime means it lives the length of the program.  It will
        // always be available.  It's mostly from literals, but it crops up in
        // other cases too.
        let literal_str: &'static str = "data";

        // Rule 1: non-static lifetimes are generics in functions/types/impls.
        fn foo1<'a>(s: &'a str) -> usize {
            s.len()
        }

        // Rule 2: Similar to type generics, the compiler solves lifetimes based on function calls.
        // QUIZ: In the next call to foo1, what does 'a resolve to?
        assert_eq!(foo1(literal_str), 4);

        // Rule 3: Each input parameter has a different lifetime by default
        fn foo2<'a, 'b>(s1: &'a str, s2: &'b str) -> usize {
            s1.len() + s2.len()
        }

        // Elision 1: If there is no lifetime in the return value, the input lifetimes _do not matter_.
        // Because of this, the compiler trivially solves this and elides them.
        fn foo3(s1: &str, s2: &str) -> usize {
            s1.len() + s2.len()
        }

        // Rule 4: Any lifetime in the return type _must_ be in the input type.
        fn first_str<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str {
            s1
        }

        // Elision 2: Any input lifetime not affecting the output lifetime can be omitted or replaced with '_.
        fn first_str2<'a>(s1: &'a str, s2: &str) -> &'a str {
            s1
        }

        // Elision 3: If there is only one input and one output lifetime, they must be the same, and can be elided.
        fn first_four1<'a>(s1: &'a str) -> Option<&'a str> {
            s1.get(0..4)
        }
        fn first_four2(s1: &str) -> Option<&str> {
            s1.get(0..4)
        }
    }
}
