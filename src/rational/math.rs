use super::Rational;
use std::cmp::Ordering;

impl Rational {
    pub fn invert(&self) -> Rational {
        Rational {
            signed: self.signed,
            numerator: self.denominator.clone(),
            denominator: self.numerator.clone(),
        }
    }

    pub fn invert_into(&mut self) {
        std::mem::swap(&mut self.numerator, &mut self.denominator);
    }

    fn add_unsigned(&self, other: &Self) -> Rational {
        // ignore sign on other
        //  a/b + c/d =>
        // (a * d)/(b * d) + (c * b)/(b * d) =>
        // (a * d) + (c * b)/(b * d)
        let mut res = Rational {
            signed: self.signed,
            numerator: self.numerator.mul_with(&other.denominator) +
                self.denominator.mul_with(&other.numerator),
            denominator: self.denominator.mul_with(&other.denominator),
        };
        let gcd = res.numerator.gcd(&res.denominator);
        if gcd > 1u32.into() {
            let _ = res.numerator.div_mod_into(&gcd);
            let _ = res.denominator.div_mod_into(&gcd);
        }
        res
    }

    fn add_unsigned_into(&mut self, other: &Self) {
        // ignore sign on other
        //  a/b + c/d =>
        // (a * d)/(b * d) + (c * b)/(b * d) =>
        // (a * d) + (c * b)/(b * d)

        self.numerator = self.numerator.mul_with(&other.denominator) +
            self.denominator.mul_with(&other.numerator);
        self.denominator = self.denominator.mul_with(&other.denominator);

        let gcd = self.numerator.gcd(&self.denominator);
        if gcd > 1u32.into() {
            let _ = self.numerator.div_mod_into(&gcd);
            let _ = self.denominator.div_mod_into(&gcd);
        }
    }

    pub fn add_to(&self, other: &Self) -> Rational {
        if self.signed == other.signed {
            self.add_unsigned(&other)
        } else if self.signed {
            // -a + b => b - a
            other.sub_from_unsigned(&self)
        } else {
            // a +(-b) => a - b
            self.sub_from_unsigned(&other)
        }
    }


    pub fn add_into(&mut self, other: &Self) {
        if self.signed == other.signed {
            //  a/b + c/d =>
            // (a * d)/(b * d) + (c * b)/(b * d) =>
            // (a * d) + (c * b)/(b * d)

            let mut numerator = self.numerator.mul_with(&other.denominator) +
                self.denominator.mul_with(&other.numerator);
            let mut denominator = self.denominator.mul_with(&other.denominator);

            let gcd = numerator.gcd(&denominator);
            if gcd > 1u32.into() {
                let _ = numerator.div_mod_into(&gcd);
                let _ = denominator.div_mod_into(&gcd);
            }
            self.numerator = numerator;
            self.denominator = denominator;
        } else if self.signed {
            // -a + b => b - a
            *self = other.sub_from_unsigned(&self);
        } else {
            // a +(-b) => a - b
            self.sub_from_unsigned_into(&other);
        }
    }

    fn sub_from_unsigned(&self, other: &Self) -> Rational {
        // ignore sign of other
        //  a/b - c/d =>
        // (a * d)/(b * d) - (c * b)/(b * d) =>
        // (a * d) - (c * b)/(b * d)
        let s_num = self.numerator.mul_with(&other.denominator);
        let o_num = other.numerator.mul_with(&self.denominator);
        let mut res = match s_num.cmp(&o_num) {
            Ordering::Greater => {
                Rational {
                    signed: self.signed,
                    numerator: s_num.sub_from(&o_num),
                    denominator: self.denominator.mul_with(&other.denominator),
                }
            }
            Ordering::Less => {
                // sign reversal
                Rational {
                    signed: !self.signed,
                    numerator: o_num.sub_from(&s_num),
                    denominator: self.denominator.mul_with(&other.denominator),
                }
            }
            Ordering::Equal => {
                Rational::new()
            }
        };
        let gcd = res.numerator.gcd(&res.denominator);
        if gcd > 1u32.into() {
            let _ = res.numerator.div_mod_into(&gcd);
            let _ = res.denominator.div_mod_into(&gcd);
        }
        res
    }

    fn sub_from_unsigned_into(&mut self, other: &Self) {
        // ignore sign of other
        //  a/b - c/d =>
        // (a * d)/(b * d) - (c * b)/(b * d) =>
        // (a * d) - (c * b)/(b * d)
        let s_num = self.numerator.mul_with(&other.denominator);
        let o_num = other.numerator.mul_with(&self.denominator);
        match s_num.cmp(&o_num) {
            Ordering::Greater => {
                self.numerator = s_num.sub_from(&o_num);
                self.denominator = self.denominator.mul_with(&other.denominator);
            }
            Ordering::Less => {
                // sign reversal
                self.signed = !self.signed;
                self.numerator = o_num.sub_from(&s_num);
                self.denominator = self.denominator.mul_with(&other.denominator);
            }
            Ordering::Equal => {
                *self = Rational::new()
            }
        }
        let gcd = self.numerator.gcd(&self.denominator);
        if gcd > 1u32.into() {
            let _ = self.numerator.div_mod_into(&gcd);
            let _ = self.denominator.div_mod_into(&gcd);
        }
    }

    pub fn sub_from(&self, other: &Self) -> Rational {
        if self.signed == other.signed {
            self.sub_from_unsigned(&other)
        } else if self.signed {
            // -a -b => -(a + b)
            self.add_unsigned(&other)
        } else {
            // a -(-b) => a + b
            other.add_unsigned(&self)
        }
    }

    pub fn sub_into(&mut self, other: &Self) {
        if self.signed == other.signed {
            self.sub_from_unsigned_into(&other);
        } else if self.signed {
            // -a -b => -(a + b)
            self.add_unsigned_into(&other)
        } else {
            // a -(-b) => a + b
            *self = other.add_unsigned(&self)
        }
    }

    pub fn mul_by(&self, other: &Self) -> Rational {
        let mut res = Rational {
            signed: self.signed ^ other.signed,
            numerator: self.numerator.mul_with(&other.numerator),
            denominator: self.denominator.mul_with(&other.denominator),
        };

        let gcd = res.numerator.gcd(&res.denominator);
        if gcd > 1u32.into() {
            res.numerator.div_mod_into(&gcd);
            res.denominator.div_mod_into(&gcd);
        }
        res
    }

    pub fn mul_into(&mut self, other: &Self) {
        self.signed = self.signed ^ other.signed;
        self.numerator.mul_into(&other.numerator);
        self.denominator.mul_into(&other.denominator);
        let gcd = self.numerator.gcd(&self.denominator);
        if gcd > 1u32.into() {
            self.numerator.div_mod_into(&gcd);
            self.denominator.div_mod_into(&gcd);
        }
    }
}
