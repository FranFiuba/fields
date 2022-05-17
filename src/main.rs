use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug)]
pub struct FieldElement {
    num: u32,
    prime: u32,
}

impl FieldElement {
    pub fn new(num: u32, prime: u32) -> Self {
        if num >= prime {
            panic!("Num {} not in field range 0 to {}", num, prime);
        }

        Self { num, prime }
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different field");
        }
        let num: u32 = (self.num + other.num) % self.prime;
        let prime: u32 = self.prime;
        Self { num, prime }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot multiplicate two numbers in different field");
        }
        let num: u32 = (self.num * other.num) % self.prime;
        let prime: u32 = self.prime;
        Self { num, prime }
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "number: {}, prime: {}", self.num, self.prime)
    }
}

fn main() {
    let field: FieldElement = FieldElement::new(10, 11);
    let other_field: FieldElement = FieldElement::new(10, 11);
    let an_other_field: FieldElement = FieldElement::new(20, 21);

    assert_eq!(field, other_field);
    assert_ne!(field, an_other_field);

    let sum_elem1: FieldElement = FieldElement::new(1, 10);
    let sum_elem2: FieldElement = FieldElement::new(1, 10);
    let sum_total_hardcoded: FieldElement = FieldElement::new(2, 10);
    let sum_total: FieldElement = sum_elem1.add(sum_elem2);

    assert_eq!(sum_total, sum_total_hardcoded);

    let mul_elem1: FieldElement = FieldElement::new(2, 10);
    let mul_elem2: FieldElement = FieldElement::new(2, 10);
    let mul_total_hardcoded: FieldElement = FieldElement::new(4, 10);
    let mul_total: FieldElement = mul_elem1.mul(mul_elem2);

    assert_eq!(mul_total, mul_total_hardcoded);
}
