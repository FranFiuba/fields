use std::fmt;


#[derive(Debug)]
pub struct FieldElemnt {
    num: u32,
    prime: u32
}


impl FieldElemnt {
    pub fn new(num: u32, prime: u32) -> Self{
        if num >= prime {
         panic!("Num {} not in field range 0 to {}", num, prime);
        }

        Self {num, prime}
    }
}


impl PartialEq for FieldElemnt {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl fmt::Display for FieldElemnt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "number: {}, prime: {}", self.num, self.prime)
    }
}

fn main() {
   let field : FieldElemnt = FieldElemnt::new(10, 11);
   let other_field : FieldElemnt = FieldElemnt::new(10, 11);
   let an_other_field: FieldElemnt = FieldElemnt::new(20, 21);

   assert_eq!(field, other_field);
   assert_ne!(field, an_other_field);

}



