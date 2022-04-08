use std::cmp::Ordering;
use std::cmp::min;
use std::mem::swap;
// #[macro_use]
use lazy_static::lazy_static;

use super::{BigUInt, BLOCK_MASK, Block, BLOCK_SIZE, BIT_65};

lazy_static! {
    static ref  BIT32_AS_F64: f64 = 2.0f64.powi(32);
    static ref  BIT64_AS_F64: f64 = 2.0f64.powi(64);
}

impl BigUInt {
    /// Add to self and return the result.
    ///
    /// Due to BigUInt not being able to implement the Copy trait and the std::ops::Add trait
    /// consuming the right hand side operator, the use of + can be inefficient having to clone
    /// the right hand side operator.
    /// This function works around that restriction, it is used by the std::ops::Add implementation
    ///
    /// # Arguments
    /// * other - the value to be added
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    ///  let mut bi = BigUInt::from_u128(0xF0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0);
    ///  assert_eq!(bi.add_to(&BigUInt::from_u32(0x0F0F)).to_hex_string(),"F0F0F0F0F0F0F0F0F0F0F0F0F0F0FFFF");
    /// ```
    #[inline]
    pub fn add_to(&self, other: &BigUInt) -> BigUInt {
        // eprintln!("add_assign({},{})", self.to_hex_string(), other.to_hex_string());
        if other.is_zero() {
            self.clone()
        } else if self.is_zero() {
            other.clone()
        } else {
            let mut overflow = false;
            let mut bits = vec![];
            for (block1, block2) in self.bits.iter().zip(other.bits.iter()) {
                let res = *block1 as u128 + *block2 as u128 + if overflow { 1 } else { 0 };
                bits.push((res & BLOCK_MASK as u128) as Block);
                overflow = res & 0x10000000000000000 == 0x10000000000000000;
            }
            // eprintln!("add_assign({},{}) after first loop, overflow: {}", self.to_hex_string(), other.to_hex_string(), overflow);
            if self.bits.len() > other.bits.len() {
                if overflow {
                    for block in &self.bits[other.bits.len()..] {
                        let res = *block as u128 + 1;
                        bits.push((res & BLOCK_MASK as u128) as Block);
                        overflow = res & 0x10000000000000000 == 0x10000000000000000;
                        if !overflow {
                            break;
                        }
                    }
                }
                if overflow {
                    bits.push(1);
                } else {
                    bits.extend_from_slice(&self.bits[bits.len()..]);
                }
            } else if other.bits.len() > self.bits.len() {
                if overflow {
                    for block in &other.bits[self.bits.len()..] {
                        let res = *block as u128 + 1;
                        bits.push(res as u64);
                        overflow = res & 0x10000000000000000 == 0x10000000000000000;
                        if !overflow {
                            break;
                        }
                    }
                }
                if overflow {
                    bits.push(1);
                } else {
                    bits.extend_from_slice(&other.bits[bits.len()..]);
                }
            } else if overflow {
                bits.push(1);
            }

            let length = if overflow {
                (bits.len() - 1) * BLOCK_SIZE + 1
            } else {
                bits.len() * BLOCK_SIZE
            };
            let mut res = BigUInt {
                length,
                bits,
            };
            if !overflow {
                res.trim();
            }
            res
        }
    }

    /// Add to self and store the result in self.
    ///
    /// Due to BigUInt not being able to implement the Copy trait and the std::ops::AddAssign trait
    /// consuming the right hand side operator, the use of += can be inefficient having to clone
    /// the right hand side operator.
    /// This function works around that restriction, it is used by the std::ops::AddAssign implementation
    ///
    /// # Arguments
    /// * other - the value to be added
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let mut bi = BigUInt::from_u128(0xF0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0);
    /// bi.add_into(&BigUInt::from_u32(0x0F0F));
    /// assert_eq!(bi.to_hex_string(),"F0F0F0F0F0F0F0F0F0F0F0F0F0F0FFFF");
    /// ```
    #[inline]
    pub fn add_into(&mut self, other: &BigUInt) {
        // eprintln!("add_assign({},{})", self.to_hex_string(), other.to_hex_string());
        if self.is_zero() {
            self.length = other.length;
            self.bits = other.bits.clone()
        } else if other.is_zero() {} else {
            let mut overflow = false;
            for (block1, block2) in self.bits.iter_mut().zip(other.bits.iter()) {
                let res = *block1 as u128 + *block2 as u128 + if overflow { 1 } else { 0 };
                *block1 = (res & BLOCK_MASK as u128) as Block;
                overflow = res & 0x10000000000000000 == 0x10000000000000000;
            }
            // eprintln!("add_assign({},{}) after first loop, overflow: {}", self.to_hex_string(), other.to_hex_string(), overflow);
            if self.bits.len() > other.bits.len() {
                if overflow {
                    for block in &mut self.bits[other.bits.len()..] {
                        let res = *block as u128 + 1;
                        *block = (res & BLOCK_MASK as u128) as Block;
                        overflow = res & 0x10000000000000000 == 0x10000000000000000;
                        if !overflow {
                            break;
                        }
                    }
                    if overflow {
                        self.bits.push(1);
                    }
                }
            } else if other.bits.len() > self.bits.len() {
                if overflow {
                    for block in &other.bits[self.bits.len()..] {
                        let res = *block as u128 + 1;
                        self.bits.push(res as u64);
                        overflow = res & 0x10000000000000000 == 0x10000000000000000;
                        if !overflow {
                            break;
                        }
                    }
                }
                if overflow {
                    self.bits.push(1);
                } else {
                    self.bits.extend_from_slice(&other.bits[self.bits.len()..]);
                }
            } else if overflow {
                self.bits.push(1);
            }
            if overflow {
                self.length = (self.bits.len() - 1) * BLOCK_SIZE + 1;
            } else {
                self.trim()
            }
        }
    }

    /// Subtract other from self and return the result in a new BigUInt.
    /// Due to BigUInt not being able to implement the Copy trait and the std::ops::Sub trait
    /// consuming the right hand side operator the use of - can be inefficient, having to clone
    /// the right hand side operator.
    /// This function works around that restriction, it is used by the std::ops::Sub implementation
    ///
    /// # Arguments
    /// * other - the value to be subtracted
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    ///  let mut bi = BigUInt::from_u128(0xF0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0);
    ///  let res = bi.sub_from(&BigUInt::from_u32(0xF0F0));
    ///  assert_eq!(res.to_hex_string(),"F0F0F0F0F0F0F0F0F0F0F0F0F0F00000");
    /// ```
    #[inline]
    pub fn sub_from(&self, other: &Self) -> Self {
        match (self).cmp(&other) {
            Ordering::Less => panic!("integer underflow"),
            Ordering::Equal => BigUInt::new(),
            Ordering::Greater => {
                let mut bits = vec![];
                let mut overflow = false;
                for (block1, block2) in self.bits.iter().zip(other.bits.iter()) {
                    let mut work = *block1 as u128;
                    if overflow {
                        if work > 0 {
                            work -= 1;
                            overflow = false;
                        } else {
                            work = BLOCK_MASK as u128;
                            overflow = true;
                        }
                    }

                    if work < *block2 as u128 {
                        overflow = true;
                        work = work + BIT_65 - *block2 as u128;
                    } else {
                        work -= *block2 as u128;
                    }

                    bits.push(work as u64);
                }

                if overflow {
                    for block in &self.bits[other.bits.len()..] {
                        if *block > 0 {
                            bits.push(*block - 1);
                            overflow = false;
                            break;
                        } else {
                            bits.push(BLOCK_MASK);
                        }
                    }
                    assert!(!overflow);
                } else {
                    bits.extend_from_slice(&self.bits[other.bits.len()..]);
                }

                let mut res = BigUInt {
                    length: bits.len() * BLOCK_SIZE,
                    bits,
                };
                res.trim();
                res
            }
        }
    }


    /// Subtract from self and store the result in self.
    /// Due to BigUInt not being able to implement the Copy trait and the std::ops::SubAssign trait
    /// consuming the right hand side operator the use of -= can be inefficient, having to clone
    /// the right hand side operator.
    /// This function works around that restriction, it is used by the std::ops::SubAssign implementation
    ///
    /// # Arguments
    /// * other - the value to be subtracted
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    ///  let mut bi = BigUInt::from_u128(0xF0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0);
    ///  bi.sub_into(&BigUInt::from_u32(0xF0F0));
    ///  assert_eq!(bi.to_hex_string(),"F0F0F0F0F0F0F0F0F0F0F0F0F0F00000");
    /// ```
    #[inline]
    pub fn sub_into(&mut self, other: &BigUInt) {
        // eprintln!("BigUInt::sub_from_into({:?},{:?})", self, other);
        // TODO: find out when trim is required, instead of doing it generally
        match (*self).cmp(other) {
            Ordering::Less => panic!("integer underflow"),
            Ordering::Equal => {
                self.length = 0;
                self.bits.clear();
            }
            Ordering::Greater => {
                // the borrowed bit from the last block
                let mut borrowed_bit = false;
                // go through the lower blocks common to both
                for (block1, block2) in self.bits.iter_mut().zip(other.bits.iter()) {
                    let mut register = *block1 as u128;
                    if borrowed_bit {
                        if register > 0 {
                            register -= 1;
                            borrowed_bit = false;
                        } else {
                            register = BLOCK_MASK as u128;
                            // borrowed_bit stays set
                        }
                    }

                    let subtract = *block2 as u128;
                    if register < subtract {
                        borrowed_bit = true;
                        register = BIT_65 + register - subtract;
                    } else {
                        register -= subtract;
                    }

                    *block1 = register as u64;
                }
                if borrowed_bit {
                    for block in &mut self.bits[other.bits.len()..] {
                        if *block > 0 {
                            *block -= 1;
                            borrowed_bit = false;
                            break;
                        } else {
                            *block = BLOCK_MASK;
                        }
                    }
                    assert!(!borrowed_bit, "borrowed bit not settled! self: {:?}, other: {:?}", self, other);
                }
                self.trim();
            }
        }
    }

    /// Multiply Divide self with another BigUInt and return the result.
    ///
    /// Due to BigUInt not being able to implement the Copy trait and the std::ops::Mul trait
    /// consuming the right hand side operator, the use of * can be inefficient, having to clone
    /// the right hand side operator.
    /// This function works around that restriction, it is used by the std::ops::Mul implementation
    ///
    /// # Arguments
    /// * other - the multiplyer
    ///
    /// # Returns
    ///
    /// The result of the multiplication
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u32(0x80000000);
    /// let res = bi.mul_with(&BigUInt::from_u32(0x3000));
    ///
    /// assert_eq!(res.to_u64(), Some(0x180000000000));
    /// ```
    #[inline]
    pub fn mul_with(&self, other: &Self) -> BigUInt {
        if self.is_zero() || other.is_zero() {
            BigUInt::new()
        } else {
            let mut res_list = vec![];
            for (idx1, block1) in other.bits.iter().enumerate() {
                for (idx2, block2) in self.bits.iter().enumerate() {
                    let mut res = BigUInt::from_u128(*block1 as u128 * *block2 as u128);
                    res <<= (idx1 + idx2) * BLOCK_SIZE;
                    res_list.push(res);
                }
            }
            let mut sum = BigUInt::new();
            for res in res_list {
                sum += res;
            }
            sum
        }
    }

    pub fn powi(&self, power: u32) -> BigUInt {
        let mut res = self.clone();
        for _ in 1..power {
            res.mul_into(self);
        }
        res
    }

    pub fn gcd(&self, other: &Self) -> BigUInt {
        // Binary GCD algorithm, see https://en.wikipedia.org/wiki/Binary_GCD_algorithm
        // Base cases: gcd(n, 0) = gcd(0, n) = n

        let mut u = self.clone();
        let mut v = other.clone();

        if u.is_zero() {
            return v;
        } else if v.is_zero() {
            return u;
        }

        // Using identities 2 and 3:
        // gcd(2ⁱ u, 2ʲ v) = 2ᵏ gcd(u, v) with u, v odd and k = min(i, j)
        // 2ᵏ is the greatest power of two that divides both u and v
        let i = u.trailing_zeros() as usize;
        u >>= i;
        let j = v.trailing_zeros() as usize;
        v >>= j;
        let k = min(i, j);

        loop {
            // u and v are odd at the start of the loop
            debug_assert!(u.is_odd(), "u = {} is even", &u);
            debug_assert!(v.is_odd(), "v = {} is even", &v);

            // Swap if necessary so u <= v
            if u > v {
                swap(&mut u, &mut v);
            }
            // u and v are still both odd after (potentially) swapping

            // Using identity 4 (gcd(u, v) = gcd(|v-u|, min(u, v))
            v.sub_into(&u);
            // v is now even, but u is unchanged (and odd)

            // Identity 1: gcd(u, 0) = u
            // The shift by k is necessary to add back the 2ᵏ factor that was removed before the loop
            if v.is_zero() {
                return u << k;
            }

            // Identity 3: gcd(u, 2ʲ v) = gcd(u, v) (u is known to be odd)
            v >>= v.trailing_zeros() as usize;
            // v is now odd again
        }
    }

    /// Multiply Divide self with another BigUInt and store the result in self.
    ///
    /// Due to BigUInt not being able to implement the Copy trait and the std::ops::MulAssign trait
    /// consuming the right hand side operator, the use of *= can be inefficient, having to clone
    /// the right hand side operator.
    /// This function works around that restriction, it is used by the std::ops::MulAssign implementation
    ///
    /// # Arguments
    /// * other - the multiplyer
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let mut bi = BigUInt::from_u32(0x80000000);
    /// bi.mul_into(&BigUInt::from_u32(0x3000));
    ///
    /// assert_eq!(bi.to_u64(), Some(0x180000000000));
    /// ```
    #[inline]
    pub fn mul_into(&mut self, other: &Self) {
        if self.is_zero() {} else if other.is_zero() {
            self.length = 0;
            self.bits.clear();
        } else {
            let mut res_list = vec![];
            for (idx1, block1) in other.bits.iter().enumerate() {
                for (idx2, block2) in self.bits.iter().enumerate() {
                    let mut res = BigUInt::from_u128(*block1 as u128 * *block2 as u128);
                    res <<= (idx1 + idx2) * BLOCK_SIZE;
                    res_list.push(res);
                }
            }
            let mut sum = BigUInt::new();
            for res in res_list {
                sum += res;
            }
            self.length = sum.length;
            self.bits = sum.bits
        }
    }


    /// Divide self by a divisor, return the result and the modulo.
    /// Due to BigUInt not being able to implement the Copy trait and the std::ops::Div trait
    /// consuming the right hand side operator the use of / can be inefficient, having to clone
    /// the right hand side operator.
    /// This function works around that restriction, it is used by the std::ops::Div implementation
    ///
    /// # Arguments
    /// * other - the divisor
    ///
    /// # Returns
    ///
    /// A tuple containing the result of the division and the modulo
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u32(0x80000000);
    /// let (quotient,modulo) = bi.div_mod(&BigUInt::from_u32(0x3000));
    /// assert_eq!(quotient.to_hex_string(), "2AAAA");
    /// assert_eq!(modulo.to_hex_string(), "2000");
    /// ```
    #[inline]
    pub fn div_mod(&self, other: &BigUInt) -> (BigUInt, BigUInt) {
        assert!(!other.is_zero(), "Division by zero");
        match (*self).cmp(other) {
            Ordering::Less => {
                (BigUInt::new(), self.clone())
            }
            Ordering::Equal => {
                (BigUInt::from_u32(1), BigUInt::new())
            }
            Ordering::Greater => {
                /*
                if D = 0 then error(DivisionByZeroException) end
                    Q := 0                  -- Initialize quotient and remainder to zero
                    R := 0
                    for i := n − 1 .. 0 do  -- Where n is number of bits in N
                    R := R << 1           -- Left-shift R by 1 bit
                    R(0) := N(i)          -- Set the least-significant bit of R equal to bit i of the numerator
                    if R ≥ D then
                        R := R − D
                        Q(i) := 1
                    end
                end
                */
                let mut res = BigUInt::new();
                let mut modulo = BigUInt::new();
                for idx in (0..self.length).rev() {
                    modulo <<= 1;
                    modulo.set(0, self.get(idx).expect("Unexpected bit index out of range"));
                    if modulo >= *other {
                        modulo.sub_into(other);
                        res.set(idx, true);
                    }
                }
                (res, modulo)
            }
        }
    }

    /// Divide self by a divisor, return the result.
    /// Due to BigUInt not being able to implement the Copy trait and the std::ops::Div trait
    /// consuming the right hand side operator the use of / can be inefficient, having to clone
    /// the right hand side operator.
    /// This function works around that restriction, it is used by the std::ops::Div implementation
    ///
    /// # Arguments
    /// * other - the divisor
    ///
    /// # Returns
    ///
    /// The result of the division
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u32(0x80000000);
    /// let quotient = bi.div_by(&BigUInt::from_u32(0x3000));
    /// assert_eq!(quotient.to_hex_string(), "2AAAA");
    /// ```
    #[inline]
    pub fn div_by(&self, other: &BigUInt) -> BigUInt {
        let (div_res, _) = self.div_mod(other);
        div_res
    }

    /// Divide self by a divisor, store the result in self and return the modulo.
    /// Due to BigUInt not being able to implement the Copy trait and the std::ops::DivAssign trait
    /// consuming the right hand side operator the use of /= can be inefficient, having to clone
    /// the right hand side operator.
    /// This function works around that restriction, it is used by the std::ops::DivAssign implementation
    ///
    /// # Arguments
    /// * other - the divisor
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let mut bi = BigUInt::from_u32(0x80000000);
    /// let modulo = bi.div_mod_into(&BigUInt::from_u32(0x3000));
    /// assert_eq!(bi.to_hex_string(), "2AAAA");
    /// assert_eq!(modulo.to_hex_string(), "2000");
    /// ```
    #[inline]
    pub fn div_mod_into(&mut self, other: &BigUInt) -> BigUInt {
        assert!(!other.is_zero(), "Division by zero");
        match (*self).cmp(other) {
            Ordering::Less => {
                let res = self.clone();
                self.length = 0;
                self.bits.clear();
                res
            }
            Ordering::Equal => {
                self.length = 1;
                self.bits.resize(1, 0);
                self.bits[0] = 1;
                BigUInt::new()
            }
            Ordering::Greater => {
                let mut res = BigUInt::new();
                let mut modulo = BigUInt::new();
                for idx in (0..self.length).rev() {
                    modulo <<= 1;
                    modulo.set(0, self.get(idx).expect("Unexpected bit index out of range"));
                    if modulo >= *other {
                        modulo.sub_into(other);
                        res.set(idx, true);
                    }
                }
                self.length = res.length;
                self.bits = res.bits;
                modulo
            }
        }
    }

    pub fn to_f64(&self) -> Result<f64,String> {
        // TODO: what to do with numbers that cannot be held in f64
        if self.is_zero() {
            Ok(0.0)
        } else {
            let mut register = 0f64;

            self.bits.iter().rev().for_each(|block| {
                register = register * *BIT32_AS_F64 + f64::from((*block >> 32) as u32);
                register = register * *BIT32_AS_F64 + f64::from((*block & 0xFFFFFFFFu64) as u32);
            });
            if register.is_nan() {
                Err(format!("{:?} produced an invalid f64", self))
            } else if register.is_infinite() {
                Err(format!("{:?} produced an infinite f64", self))
            } else {
                Ok(register)
            }
        }
    }

    pub fn from_f64(src: f64) -> BigUInt {
        // TODO: check for optimization / error reduction
        if src < 1.0 {
            BigUInt::new()
        } else {
            let mut bits = Vec::new();
            let mut register = src;
            while register >= 1.0 {
                bits.push((register % *BIT64_AS_F64) as u64);
                register /= *BIT64_AS_F64;
            }
            let mut res = BigUInt{
                length: bits.len() * BLOCK_SIZE,
                bits
            };
            res.trim();
            res
        }
    }

}
