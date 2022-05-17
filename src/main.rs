#[derive(Debug)]
pub struct FieldElemnt {
    num: u32,
    prime: u32
}


impl FieldElemnt {
    pub fn new(num: u32, prime: u32) -> Self{
        Self {num, prime}
    }
}


impl PartialEq for FieldElemnt {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

fn main() {
   let field : FieldElemnt = FieldElemnt::new(10, 3);
   let other_field : FieldElemnt = FieldElemnt::new(10, 3);
   let an_other_field: FieldElemnt = FieldElemnt::new(20, 2);

   assert_eq!(field, other_field);
   assert_ne!(field, an_other_field);
}



