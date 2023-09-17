#![cfg(test)]
#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(inherent_associated_types)]

use iter_all::IterAll;
use iter_all_proc::IterAll;

mod generics_test;

#[derive(IterAll)]
enum Enum {
    A,
    B,
    C,
    D,
}

impl Enum {
    fn name(&self) -> &'static str {
        match self {
            Enum::A => "A",
            Enum::B => "B",
            Enum::C => "C",
            Enum::D => "D",
        }
    }
}

#[test]
fn test() {
    let mut sargovnost = vec![];

    Enum::iter_all(|val| sargovnost.push(val.name()));

    assert_eq!(vec!["A", "B", "C", "D",], sargovnost);
}
