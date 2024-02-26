// We'll need a basic struct for ownership.  We'll get into them more soon.

// derive macros let us give canonical behavior easily.  Ignore until we see traits.
#[derive(Debug)]
struct Foo(i32);

#[cfg(test)]
mod tests {
    // Make it easy to refer to Foo.
    use super::Foo;

    #[test]
    fn test_ownership_move_semantics() {
        let x = Foo(0);
        let y = x;
        // We cannot use x anymore, it's been moved
        // EXERCISE: Uncomment this and try to run the test
        // println!("test_ownership_move_semantics {x:?}");
        // y has ownership of the original Foo value
        println!("test_ownership_move_semantics {y:?}");
        // NB: {y:?} prints the Debug info for y.  Not generally for end-user consumption.
    }

    fn pass_by_move(foo: Foo) {
        println!("pass_by_move {foo:?}");
    }

    #[test]
    fn test_ownership_pass_by_move() {
        let x = Foo(0);
        pass_by_move(x);
        // We've given x by move to pass_by_move; we can't access it
        // EXERCISE: Uncomment and try to run the test
        // println!("test_pass_by_move {x:?}");
    }

    fn pass_by_ref(foo: &Foo) {
        println!("pass_by_ref {foo:?}");
    }

    #[test]
    fn test_ownership_pass_by_ref() {
        let x = Foo(0);
        pass_by_ref(&x);
        println!("test_pass_by_ref x1 {x:?}");
        let y = &x;
        println!("test_pass_by_ref x2 {x:?}");
        println!("test_pass_by_ref y {y:?}");
    }

    fn pass_by_mut(foo: &mut Foo) {
        // This accesses the 0th element; we'll dig into this next module.
        foo.0 += 1;
        println!("pass_by_mut {foo:?}");
    }

    #[test]
    fn test_ownership_pass_by_mut() {
        // Need to declare the binding as mutable to allow mutable borrows.
        let mut x = Foo(0);
        let y = &mut x;
        pass_by_mut(y);
        // Can't have & borrows when a &mut borrow exists; y is still alive!
        // pass_by_ref(&x);
        // Can't have multiple &mut borrows; y is still alive!
        // pass_by_mut(&mut x);
        println!("test_pass_by_mut y {y:?}");
        // y is not used anymore, it's gone and we can use &x and &mut x
        pass_by_ref(&x);
        pass_by_mut(&mut x);
    }

    #[test]
    fn test_ownership_swap() {
        // Mutable references also allow us to swap things.
        let mut x = Foo(0);
        let y = &mut x;
        // * dereferences (like C), allowing us to "swap pointers"
        *y = Foo(1);
        assert_eq!(x.0, 1);
        // EXERCISE: do this with std::mem::swap or std::mem::replace
    }

    #[test]
    fn test_ownership_strings() {
        // Rust is a bit persnickity about different string types.
        // Defining things the "natural" way gives you an immutable literal &str
        // that's valid for the lifetime of the whole program ("static").
        let s1: &'static str = "literal";

        // To have an owned string that you can mutate, you need String.
        let mut s: String = String::from("different literal");
        let s_mut: &mut String = &mut s;
        // '!' is the (4-byte) unicode literal character
        s_mut.push('!');
        // &String is always used as &str.  &String is basically never used.

        fn print_me(me: &str) {
            println!("Me: {me}");
        }
        print_me(&s);

        // But String and str are pretty interoperable:
        assert_eq!(s, "different literal!");
    }

    // Pro-tip: You can define structs in all sorts of scopes.
    #[derive(Debug, Clone)]
    struct Bar(String);

    #[test]
    fn test_ownership_clone() {
        // You can make things simpler in a lot of cases by cloning an object.  If the object
        // `foo` implements Clone, calling `foo.clone()` will make a deep copy of it.
        // You can skip a lot of borrow checker/lifetime pain with this trick.
        let bar = Bar(String::from("don't change me!"));
        let mut bar2 = bar.clone();
        bar2.0.replace_range(0..5, "please");
        assert_eq!(bar.0, "don't change me!");
        assert_eq!(bar2.0, "please change me!");
    }

    #[test]
    fn exercise_ownership_move() {
        let bar = Bar(String::from("I like to eat crackers"));

        // Pro-tip: r"" is a raw string; newlines are ok.
        // r#""# allows quotes.
        // r##""## allows quotes and #s, etc.
        unimplemented!(
            r#"
            Make a function that takes a Bar by move, and returns a Bar with "... at work!" appended to its interior.
            Search for `rustlang String` for the String API.
            But be warned -- Strings can be a rabbit hole, just learn what you need.
        "#
        );

        // Don't touch this line.
        assert_eq!(&bar.0, "I like to eat crackers... at work!");
    }

    #[test]
    fn exercise_ownership_ref() {
        let bar = Bar(String::from("I like to eat crackers"));

        unimplemented!(
            r#"
            Make a function that takes a Bar with no return value, and modifies
            it in-place to append "...at work!".
        "#
        );

        // Don't touch this line.
        assert_eq!(&bar.0, "I like to eat crackers... at work!");
    }

    #[test]
    fn exercise_ownership_copy() {
        // Like many languages, Rust lets you pass-by-value fixed size primitives instead of moving
        let x = 1;
        let y = x;
        println!("exercise_ownership_copy x {x}");

        fn pass_by_move_int(i: i32) {
            println!("pass_by_move_int {i}");
        }

        pass_by_move_int(y);
        println!("exercise_ownership_copy y {y}");

        // Rust also lets you extend these "Copy semantics" to other types!
        unimplemented!(
            r"
            EXERCISE: Read about Copy at https://doc.rust-lang.org/std/marker/trait.Copy.html
            and make a version of Foo (called FooCopy) that uses copy semantics.
            "
        );
    }
}
