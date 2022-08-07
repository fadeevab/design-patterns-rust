use crate::{TwoValuesArray, TwoValuesStruct};

pub trait Visitor {
    type Value;

    fn visit_str(&self, s: &str) -> Self::Value;
    fn visit_vec(&self, v: Vec<i32>) -> Self::Value;
}

impl Visitor for TwoValuesStruct {
    type Value = TwoValuesStruct;

    fn visit_str(&self, s: &str) -> Self::Value {
        let mut iter = s.split_ascii_whitespace();
        TwoValuesStruct {
            a: iter.next().unwrap().parse().unwrap(),
            b: iter.next().unwrap().parse().unwrap(),
        }
    }

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value {
        TwoValuesStruct { a: v[0], b: v[1] }
    }
}

impl Visitor for TwoValuesArray {
    type Value = TwoValuesArray;

    fn visit_str(&self, s: &str) -> Self::Value {
        let mut iter = s.split_ascii_whitespace();
        let mut ab = [0i32; 2];

        ab[0] = iter.next().unwrap().parse().unwrap();
        ab[1] = iter.next().unwrap().parse().unwrap();

        TwoValuesArray { ab }
    }

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value {
        let mut ab = [0i32; 2];

        ab[0] = v[0];
        ab[1] = v[1];

        TwoValuesArray { ab }
    }
}
