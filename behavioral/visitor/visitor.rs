use crate::{TwoValuesArray, TwoValuesStruct};

/// Visitor can visit one type, do conversions, and output another type.
///
/// It's not like all visitors must return a new type, it's just an example
/// that demonstrates the technique.
pub trait Visitor {
    type Value;

    /// Visits a vector of integers and outputs a desired type.
    fn visit_vec(&self, v: Vec<i32>) -> Self::Value;
}

/// Visitor implementation for a struct of two values.
impl Visitor for TwoValuesStruct {
    type Value = TwoValuesStruct;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value {
        TwoValuesStruct { a: v[0], b: v[1] }
    }
}

/// Visitor implementation for a struct of values array.
impl Visitor for TwoValuesArray {
    type Value = TwoValuesArray;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value {
        let mut ab = [0i32; 2];

        ab[0] = v[0];
        ab[1] = v[1];

        TwoValuesArray { ab }
    }
}
