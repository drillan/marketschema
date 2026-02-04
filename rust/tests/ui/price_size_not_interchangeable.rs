//! Verify that Price and Size types are not interchangeable.
//!
//! Both Price and Size wrap f64, but they represent different domain concepts
//! (price vs quantity). Direct assignment between them should fail to compile.

use marketschema::types::definitions::{Price, Size};

fn main() {
    let price = Price(100.0);
    // This should fail: Price cannot be directly assigned to Size
    let _size: Size = price;
}
