// You can make C-style enums.
#[derive(Debug, PartialEq)]
enum PrimaryColor {
    Red,
    Green,
    Blue,
}

// But the power lies in enums with different member types!
#[derive(Debug, PartialEq)]
enum Color {
    Primary(PrimaryColor),
    Rgb(u8, u8, u8),
    Grey(f32),
    Other { desc: String },
}

// enums can have methods too!
// Note that variants of enums are always public.
impl Color {
    pub fn describe(&self) -> String {
        // `match` is the equivalent of destructuring for structs
        match self {
            // Each arm has a "pattern" that handles one or more cases
            Color::Rgb(r, g, b) => format!("RGB Red: {r} Green: {g} Blue: {b}"),
            // Patterns can have match-guards can further refine
            // *scale dereferences the &f32 to F32.
            Color::Grey(scale) if *scale > 0.7 => format!("Light Grey {scale}"),
            // match returns the first pattern that triggers; scale > 0.7 will never get here
            Color::Grey(scale) => format!("Grey {scale}"),
            // nested patterns make things clearer!
            Color::Primary(PrimaryColor::Red) => format!("Primary Red"),
            Color::Primary(primary) => format!("{primary:?}"),
            // Match statements _must_ be exhaustive -- missing a case is a compiler error
            Color::Other { desc } => desc.clone(),
        }
    }

    pub fn is_greyscale(&self) -> bool {
        // Namespace these in scope just to reduce boilerplate;
        use Color::*;
        match self {
            // _ is a blackhold that indicates you don't care about the value
            Primary(_) => false,
            Grey(_) => true,
            Rgb(r, g, b) => (r == g) && (g == b),
            // _ matches anything, like `default` or `else` in switch statements
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_enum() {
        let primary = PrimaryColor::Red;
        let color = Color::Primary(primary);
        assert_eq!(color, Color::Primary(PrimaryColor::Red));
    }

    #[test]
    fn exercise_enums_struct_enum() {
        unimplemented!(
            r"
            EXERCISE: How do you construct struct enums?
            Uncomment and correct the below lines
        "
        );
        // let other = Color::Other ... ??
        // assert_eq!(other, ???);
    }

    #[test]
    fn exercise_if_let() {
        // `if let` is very useful syntactic sugar
        let color = Color::Rgb(0, 64, 124);
        if let Color::Rgb(r, g, b) = color {
            assert_eq!(r, 0);
            // EXERCISE: Fix these!
            assert_eq!(g, 0);
            assert_eq!(b, 0);
        }

        if let Color::Grey(_grey) = color {
            unreachable!("Not gonna match")
        } else {
            println!("Not grey!");
        }
    }

    #[test]
    fn exercise_enum_let_else() {
        let color = Color::Rgb(0, 64, 124);
        unimplemented!(
            r"
            EXERCISE: if-let and let combine to form let-else.
            The lines below will not compile.  Make them compile with
            let-else. (See https://fburl.com/9ck0comr).
        "
        );
        // fn get_grey(color: Color) -> f32 {
        //     let Color::Grey(grey) = color;
        //     return grey;
        // }
        // assert_eq!(get_grey(color), 0.0);
    }
}
