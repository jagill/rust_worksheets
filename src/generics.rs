#[cfg(test)]
mod tests {
    // Rust has Generic types, which are better structured and more powerful
    // than Java's Generics.  They are closer implementation to C++'s templates,
    // although they are much more limited in expressivity.  Mostly this
    // is a good thing, because it prevents the template sorcery that makes some
    // C++ code so hard to understand or debug.

    #[test]
    fn test_generics_vec() {
        // One of the main ways generics are used is to abstract a type's behavior
        // from the details of what it contains.  Vec (similar to List in other languages)
        // is a prime example of this.

        // Functions can define generics
        fn first_from_vec<T>(stuff: &mut Vec<T>) -> Option<T> {
            if stuff.is_empty() {
                None
            } else {
                Some(stuff.remove(0))
            }
        }

        // Aside: Since Rust doesn't have variadic arguments, literal Vec constructors
        // need to be macros.  We won't go into macros yet.
        // Also, unlike Java, generics can represent any type, not just non-primitives.
        let mut list: Vec<i32> = vec![1, 2, 3];

        assert_eq!(first_from_vec(&mut list), Some(1));
    }

    // Definine a type with generic arguments give us lots of power.
    enum Either<L, R> {
        Left(L),
        Right(R),
    }

    #[test]
    fn test_generics_types() {
        fn foo() -> Either<bool, i32> {
            Either::Left(true)
        }

        match foo() {
            Either::Left(b) => assert_eq!(b, true),
            Either::Right(i) => assert_eq!(i, 0),
        }
    }

    #[test]
    fn test_generics_impls() {
        // We can access generics in the impl block
        impl<L, R> Either<L, R> {
            pub fn left(&self) -> Option<&L> {
                if let Either::Left(left) = self {
                    Some(left)
                } else {
                    None
                }
            }

            pub fn right(&self) -> Option<&R> {
                if let Either::Right(right) = self {
                    Some(right)
                } else {
                    None
                }
            }
        }

        let choice: Either<bool, i32> = Either::Left(true);
        assert_eq!(choice.left(), Some(&true));
        assert_eq!(choice.right(), None);
    }

    #[test]
    fn test_generics_impls_concrete() {
        // Generics can be used in cool ways in impls. Notice we've constrained each side to be the same!
        impl<T> Either<T, T> {
            pub fn value(self) -> T {
                match self {
                    Either::Left(t) => t,
                    Either::Right(t) => t,
                }
            }
        }

        let choice: Either<i32, i32> = Either::Left(2);
        assert_eq!(choice.value(), 2);

        let other_choice: Either<i32, bool> = Either::Left(2);
        // This won't compile! `value()` isn't defined if the types are different:
        // no method named `value` found for enum `Either<i32, bool>` in the current scope
        // the method was found for
        // - `Either<T, T>`
        // assert_eq!(other_choice.value(), 2);

        // You can even specify them exactly.
        impl Either<i32, u32> {
            fn abs(&self) -> u32 {
                match self {
                    Either::Left(i) => i.abs() as u32,
                    Either::Right(u) => *u,
                }
            }
        }

        let maybe_neg = Either::Left(-1);
        assert_eq!(maybe_neg.abs(), 1);
        let other_maybe_neg: Either<i32, i32> = Either::Left(-1);
        // Won't compile!
        // assert_eq!(other_maybe_neg.abs(), 1);

        // Everything we talked about works for structs too!
        struct Foo<T, S> {
            t: T,
            s: S,
        }

        impl Foo<i32, f32> {
            pub fn multiply(&self) -> f32 {
                (self.t as f32) * self.s
            }
        }
    }

    #[test]
    fn exercise_generics_typestate() {
        // Generics are _really_ powerful, sometimes bordering on sorcery.
        // Check out what you can do here.

        use std::collections::HashMap;

        struct HttpResponseBuilder<STATE> {
            state: STATE,
        }
        struct New;
        struct Headers {
            status_code: u16,
            headers: HashMap<String, String>,
        }
        struct HttpResponse {
            status_code: u16,
            headers: HashMap<String, String>,
            body: String,
        }

        impl HttpResponseBuilder<New> {
            pub fn new() -> Self {
                HttpResponseBuilder { state: New }
            }

            pub fn status(self, code: u16) -> HttpResponseBuilder<Headers> {
                HttpResponseBuilder {
                    state: Headers {
                        status_code: code,
                        headers: HashMap::new(),
                    },
                }
            }
        }

        impl HttpResponseBuilder<Headers> {
            pub fn add_header(mut self, key: String, val: String) -> Self {
                self.state.headers.insert(key, val);
                self
            }

            pub fn body(self, contents: String) -> HttpResponse {
                let Headers {
                    status_code,
                    headers,
                } = self.state;
                HttpResponse {
                    status_code,
                    headers,
                    body: contents,
                }
            }
        }

        unimplemented!(
            r#"
            This pattern enforces at compile-type the correct order of operations.
            Build an HttpResponse `response` using HttpResponseBuilder to make
            these asserts pass.
            "#
        );

        // BUILD ME HERE

        // assert_eq!(response.status_code, 200);
        // assert_eq!(response.headers.get("foo", "bar"));
        // assert_eq!(response.body, "snoopy");
    }

    // Generics are often used with Traits, and we'll talk about them next.
}
