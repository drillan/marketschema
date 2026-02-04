//! Verify that Currency does not implement From<String>.
//!
//! Currency requires pattern validation (ISO 4217: ^[A-Z]{3}$),
//! so only TryFrom<String> is provided, not From<String>.
//! Using .into() should fail to compile.

use marketschema::types::definitions::Currency;

fn main() {
    // This should fail: Currency only implements TryFrom, not From
    let _currency: Currency = "JPY".to_string().into();
}
