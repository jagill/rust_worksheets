#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

// First let's look at how versatile Rust's "everything is an expression"
// philosophy is.
mod expressions;

// Next let's look at one of Rust's core concepts: Ownership.
// It'll be the first real hurdle you'll face.
mod ownership;

// Structs are Rust's analog to classes.  Very similar; let's start
// with structs as a way to group together data.
mod structs_data;

// Structs also allow "methods" (actually they are just functions),
// making them even more class-like.
mod structs_impl;

// Rust enums are actually Sum types and are incredible powerful.
mod enums_match;

// Billion-dollar mistake?  Not in Rust!
mod options;

// Rust's error handling is very powerful, but it takes some getting used to.
mod errors;

// Generics are unusually powerful in Rust.
mod generics;

// Traits are Rust's analog to Interfaces, and remove the need for class hierarchy.
// They interact powerfully with generics.
mod traits;

// Bonus Round: Iterators are a powerful example of traits in Rust.
mod iterators;

// Lifetimes were never supposed to be something that normal people interacted with. Hah!
mod lifetimes;

// Using closures successfully in Rust requires understanding them more deeply.
mod closures;

// Rust's async is very powerful, but still has some rough edges.  Let's dive in, level by level.
mod async_await;
