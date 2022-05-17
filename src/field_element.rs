use std::ops::{Add, Div, Mul, Sub};

pub struct FieldElement {
    elem: i64,
    prime: i64,
}

impl Add for FieldElement {
    type Output = FieldElement;
    fn add(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Elements belong to different fields");
        }
        FieldElement {
            elem: (self.elem + other.elem) % self.prime,
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;
    fn sub(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Elements belong to different fields");
        }
        FieldElement {
            elem: (self.elem - other.elem) % self.prime,
            prime: self.prime,
        }
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;
    fn mul(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Elements belong to different fields");
        }
        FieldElement {
            elem: (self.elem * other.elem) % self.prime,
            prime: self.prime,
        }
    }
}
impl Div for FieldElement {
    type Output = FieldElement;
    fn div(self, other: FieldElement) -> FieldElement {
        let prime = self.prime;
        self * other.pow(prime - 2)        
    }
}

impl FieldElement {
    pub fn new(elem: i64, prime: i64) -> FieldElement {
        FieldElement { elem, prime }
    }
    pub fn pow(self, num: i64) -> FieldElement {
        let exp = if num < 0 { num % (self.prime - 1) } else { num };
        FieldElement {
            elem: i64::pow(self.elem, exp as u32),
            prime: self.prime,
        }
    }
}
