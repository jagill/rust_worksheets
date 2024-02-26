trait IntStream {
    // Traits define behavior, like Interfaces in Java
    // Since we're defining this for consumption, all the functions are assumed public.
    fn next(&mut self) -> Option<i32>;

    // Traits can also have default implementations,
    // so implementing one method can produce many more.
    fn next_nat(&mut self) -> Option<u32> {
        while let Some(i) = self.next() {
            if i >= 0 {
                return Some(i as u32);
            }
        }
        None
    }
}

struct Count(i32);

impl Count {
    /// Return the value and increment by one.
    fn advance_value(&mut self) -> i32 {
        let value = self.0;
        self.0 += 1;
        value
    }
}

impl IntStream for Count {
    fn next(&mut self) -> Option<i32> {
        // You can refer to methods defined on the struct
        let i = self.advance_value();
        Some(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise_traits_count_from_0() {
        let mut count = Count(0);

        unimplemented!("What comes next?");
        // assert_eq!(count.next(), ???);
        // assert_eq!(count.next_nat(), ???);
    }

    #[test]
    fn exercise_traits_count_from_neg() {
        let mut count = Count(-5);

        unimplemented!("What comes next?");
        // assert_eq!(count.next(), ???);
        // assert_eq!(count.next_nat(), ???);
    }

    #[test]
    fn exercise_traits_supertraits() {
        // Traits can depend on other traits.  This is the main form of "inheritance" in Rust.
        trait EvenStream: IntStream {
            fn next_even(&mut self) -> Option<i32> {
                while let Some(i) = self.next() {
                    if i % 2 == 0 {
                        return Some(i);
                    }
                }
                None
            }
        }

        // You can blanket impl a trait if all its functions are defined in terms of its supertraits
        impl<T> EvenStream for T where T: IntStream {}

        let mut count = Count(1);
        unimplemented!("What comes next?");
        // assert_eq!(count.next_even(), ???);
        // assert_eq!(count.next_even(), ???);
    }

    #[test]
    fn exercise_traits_parameter() {
        let mut count = Count(0);

        // There are several ways to accept a type that implements a trait.  `impl Trait` is one of the easiest.
        // `impl Trait` says that you don't know what type it is, but that it implements the trait.
        // The compiler rustc actually creates one copy of this function for each type it's called with; this is
        // called 'monomorphizing' because each function takes only a defined type.
        // This is actually just syntactic sugar for generics!
        //
        // In function input parameters, this is pretty safe.
        fn read_two(stream: &mut impl IntStream) -> (Option<i32>, Option<i32>) {
            (stream.next(), stream.next())
        }

        // The compiler creates a function `read_two::<Count>` since it's called with Count.
        let output = read_two(&mut count);

        unimplemented!("What comes next?");
        // assert_eq!(output, ????);
    }

    #[test]
    fn exercise_traits_return() {
        fn make_stream() -> impl IntStream {
            Count(0)
        }

        let stream = make_stream();
        unimplemented!("What comes next?");
        // assert_eq!(stream.next(), ???);

        // BEWARE: `impl Trait` in return position means that there's a single concrete type
        // that's being returned, but we're hiding the type behind the trait.  But this
        // means you cannot conditionally return a different type!

        struct Constant(i32);
        impl IntStream for Constant {
            fn next(&mut self) -> Option<i32> {
                Some(self.0)
            }
        }

        // This does not compile!  We need higher-level powers.
        // fn make_mystery_stream(maybe: bool) -> impl IntStream {
        //     if maybe {
        //         Count(0)
        //     } else {
        //         Constant(0)
        //     }
        // }
    }

    #[test]
    fn test_traits_generic_bounds() {
        // Traits defined elsewhere can only be used if imported.
        // This prevents people from implementing a trait for your type and having
        // behavior change without you realizing it!
        use std::fmt::Display;

        // These are all the same
        fn printme_where_bound<T>(thing: &T)
        where
            T: Display,
        {
            println!("{thing}");
        }

        fn printme_short_bound<T: Display>(thing: &T) {
            println!("{thing}");
        }

        fn printme_impl(thing: &impl Display) {
            println!("{thing}");
        }

        let thing = "thang";
        printme_where_bound(&thing);
        printme_short_bound(&thing);
        printme_impl(&thing);
    }

    #[test]
    fn exercise_traits_generic_bounds_impl() {
        struct Wrapper<T> {
            inner: T,
        }
        let count_wrapper = Wrapper { inner: Count(0) };

        unimplemented!(
            r#"
            You can also bound the generics in an impl with a trait, allowing
            the methods to access the trait's methods.  The impl below will not
            compile as-is; use trait bounds with IntStream to make it compile.
            "#
        );

        // impl<T> Wrapper {
        //     fn next(&mut self) -> Option<i32> {
        //         self.inner.next()
        //     }
        // }
        // assert_eq!(count_wrapper.next(), ???);
    }

    #[test]
    fn exercise_traits_associated_constants() {
        // Sometimes you want to associate a constant with a trait.
        // HINT: https://doc.rust-lang.org/reference/items/associated-items.html
        trait BoundedIntStream: IntStream {
            const MAX: i32;

            fn bounded_next(&mut self) -> Option<i32> {
                // Remember the ? operator?
                let next = self.next()?;
                Some(next.min(Self::MAX))
            }
        }

        impl BoundedIntStream for Count {
            const MAX: i32 = 5;
        }

        let mut count = Count(4);
        unimplemented!("What comes next?");
        // assert_eq!(count.next(), ???);
        // assert_eq!(count.next(), ???);
        // assert_eq!(count.next(), ???);
    }

    #[test]
    fn exercise_traits_associated_types() {
        // Associated types are very powerful.  They allow static-time validated type interdependence.
        // They take the place of inner types in Java.  One can go pretty deep into associated types;
        // here we just want to give you a taste.
        trait Stream {
            type Output;

            fn next(&mut self) -> Option<Self::Output>;
        }

        // Forgive the use of generics before we discuss them.
        impl<T> Stream for Vec<T> {
            type Output = T;

            fn next(&mut self) -> Option<Self::Output> {
                self.pop()
            }
        }

        fn two_ints(stream: &mut impl Stream<Output = i32>) -> (Option<i32>, Option<i32>) {
            (stream.next(), stream.next())
        }

        let mut list = vec![1, 2, 3];

        unimplemented!("What comes next?");
        let output = two_ints(&mut list);
        // assert_eq!(output, ????);
        // assert_eq!(list, ???);
    }

    // Extra-credit: What if two traits define the methods of the same name?
    // Can a type implement both?  If so, how do you call the different variants?

    // Extra-credit: What if your associated type has an associated type?
    // How do you express that?

    // Extra-credit: Look up Extension Traits as a way of adding functionality
    // to types in a controlled way.
}
