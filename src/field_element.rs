use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Eq)]
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
    fn sub(self, other: FieldElement) -> Self {
        if self.prime != other.prime {
            panic!("Elements belong to different fields");
        }
        FieldElement {
            elem: (self.prime + self.elem - other.elem) % self.prime,
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
        let exp = if num < 0 { num.rem_euclid(self.prime - 1) } else { num };
        FieldElement {
            elem: i64::pow(self.elem, exp as u32) % self.prime,
            prime: self.prime,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let a = FieldElement::new(4, 19);
        assert_eq!(4, a.elem);
        assert_eq!(19, a.prime);
    }

    #[test]
    fn add_test() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(12, 13);
        let expected = FieldElement::new(6, 13);
        assert_eq!(expected, a + b);
    }

    #[test]
    fn sub_test() {
        let a = FieldElement::new(6, 13);
        let b = FieldElement::new(12, 13);
        let expected = FieldElement::new(7, 13);
        assert_eq!(expected, a - b);
    }

    #[test]
    fn mul_test() {
        let a = FieldElement::new(8, 19);
        let b = FieldElement::new(17, 19);
        let expected = FieldElement::new(3, 19);
        assert_eq!(expected, a * b);
    }

    #[test]
    fn pow_test() {
        let a = FieldElement::new(9, 19);
        let expected = FieldElement::new(7, 19);
        assert_eq!(expected, a.pow(12));
    }

    #[test]
    fn negative_pow_test() {
        // 3 ^ (-1) = 3 ^ (5 - 1 - 1) = 3 ^ 3 = 27 % 5 == 4
        let a = FieldElement::new(3, 5);
        let expected = FieldElement::new(2, 5);
        assert_eq!(expected, a.pow(-1));
    }

    #[test]
    fn div_test() {
        let a = FieldElement::new(7, 19);
        let b = FieldElement::new(5, 19);
        let expected = FieldElement::new(9, 19);
        assert_eq!(expected, a / b);
    }

}
