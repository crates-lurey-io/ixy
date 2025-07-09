//! Unlike many other 2D math libraries, this crate supports generics, and is easily extensible.
//!
//! This example shows how to use the `Pos` type with generics to add two positions together.

use ixy::{Pos, int::Int};

fn main() {
    let a = Pos::new(3, 4);
    let b = Pos::new(1, 2);
    let c = add(a, b);

    assert_eq!(c.x, 4);
    assert_eq!(c.y, 6);
}

fn add<T: Int>(a: Pos<T>, b: Pos<T>) -> Pos<T> {
    Pos {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}
