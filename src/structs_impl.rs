#[derive(Debug)]
struct Foo {
    ok: bool,
    msg: String,
}

// We add methods to a struct by an impl block
// Unlike most languages, you don't put it in the definition
impl Foo {
    // Ok, lots to unpack here:
    // 1. methods without a `self` are static methods, and invoked like Foo::new()
    // 2. `pub` means it's accessible to callers outside of the struct.
    // 3. Within impl, `Self` refers to the type, so `Foo`.
    // 4. It's convention to call a static constructor `new`.  Why use this instead of Foo { ... } directly?
    // 4A. ...because fields might be private (foreshadowing) and you can't use the {} constructor then.
    // 4B. ...because you might want to do something more complex than just assign values.
    // 4C. ...because you might want to have different fields in the public constructor than the private.
    pub fn new() -> Self {
        // Could also be `Foo { ... }
        Self {
            ok: true,
            msg: "All's good".to_owned(),
        }
        // Extra-credit: Wait, why don't we need return here?
    }

    // Methods with `&self` take an reference.  We know that these can't mutate the interior.
    // This exposes an immutable view of msg; it's Rust's equivalent of read-only members.
    // Pro-tip: We don't make getters and setters like in Java, just make those members public.
    pub fn msg(&self) -> &str {
        &self.msg
    }

    // Methods with &mut self take a mutable reference.  This is how we change state.
    pub fn make_not_ok(&mut self, new_msg: String) {
        self.ok = false;
        self.msg = new_msg;
    }

    pub fn make_ok(&mut self) {
        // methods can call private methods.
        self.reset();
    }

    fn reset(&mut self) {
        self.ok = true;
        self.msg = "All's good".to_owned();
    }

    // Methods that take `self` consume the value, with allows you to transfer ownership.
    pub fn deconstruct(self) -> (bool, String) {
        (self.ok, self.msg)
    }
}

#[cfg(test)]
mod tests {
    struct Counter {
        pub max: u32,
        curr: u32,
    }

    #[test]
    fn exercise_structs_impl_1() {
        unimplemented!(
            r"
        Implement methods for Counter:
        1. a static `new` method, taking max, creating a new Counter.
        2. a method to return the current value of `curr`.
        3. a method to increment curr.  What should happen when curr >= max?

        With these methods, create a new Counter, increment it twice, then
        uncomment the assert so that it passes.

        Pro-tip: You can add an impl block right here in this method!
        "
        );

        // assert_eq!(c.current(), 2);
    }
}
