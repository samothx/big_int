pub struct BigUInt {
    bits: Vec<bool>
}

const HEX_DIGITS: [char;16] = ['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F'];

impl BigUInt {
    pub fn from_u32(from: u32) -> BigUInt {
        let mut res = BigUInt{
            bits: Vec::new()
        };
        let mut work = from;

        let mut start = false;
        for _idx in 0..32 {
            if work & 0x80000000 == 0x80000000 {
                start = true;
                res.bits.push(true);
            } else if start {
                res.bits.push(false);
            }
            work = work << 1;
        }
        res
    }

    pub fn from_u64(from: u64) -> BigUInt {
        let mut res = BigUInt{
            bits: Vec::new()
        };
        let mut work = from;

        let mut start = false;
        for _idx in 0..64 {
            if work & 0x8000000000000000 == 0x8000000000000000 {
                start = true;
                res.bits.push(true);
            } else if start {
                res.bits.push(false);
            }
            work = work << 1;
        }
        res
    }

    pub fn is_zero(&self) -> bool {
        self.bits.is_empty()
    }

    pub  fn add(&self, other: BigUInt) -> BigUInt {
        if other.is_zero() {
            BigUInt{ bits: self.bits.clone() }
        } else {
            let mut temp = Vec::new();
            let (op1, op2) = if self.bits.len() > other.bits.len() {
                for _idx in 0..self.bits.len() - other.bits.len() {
                    temp.push(false);
                }
                temp.append(&mut other.bits.clone());
                (&self.bits, &temp)
            } else if other.bits.len() > self.bits.len() {
                for _idx in 0..other.bits.len() - self.bits.len() {
                    temp.push(false);
                }
                temp.append(&mut other.bits.clone());
                (&self.bits, &temp)
            } else {
                (&self.bits, &other.bits)
            };

            eprintln!("op1: {:?}", op1);
            eprintln!("op2: {:?}", op2);
            let mut values = Vec::new();
            let mut overflow = false;
            for (bit1, bit2)  in op1.iter().zip(op2).rev() {
                eprintln!("bit1: {} bit2: {}", bit1, bit2);
                if *bit1 {
                    if *bit2 {
                        if overflow {
                            values.push(true)
                        } else {
                            values.push(false)
                        }
                        overflow = true;
                    } else {
                        if overflow {
                            values.push(false);
                            overflow = true;
                        } else {
                            values.push(true);
                        }
                    }
                }  else {
                    if *bit2 {
                        if overflow {
                            values.push(false);
                            overflow = true;
                        } else {
                            values.push(true);
                        }
                    } else {
                        if overflow {
                            values.push(true);
                            overflow = false;
                        } else {
                            values.push(false);
                        }
                    }
                }
                eprintln!("overflow: {}, values: {:?}", overflow, values);
            }
            if overflow {
                values.push(true);
            }
            values.reverse();

            BigUInt{ bits: values }
        }
    }

    pub  fn subtract(&self, other: BigUInt) -> BigUInt {
        if self.bits.len()  < other.bits.len() {
            panic!("subtraction wraps around");
        } else {
            if other.is_zero() {
                BigUInt{ bits: self.bits.clone() }
            } else {
                let mut sub = Vec::new();
                for _idx in 0..self.bits.len() - other.bits.len() {
                    sub.push(false);
                }
                sub.append(&mut other.bits.clone());
                let mut overflow = 0;
                let mut values = Vec::new();
                for idx in (0..self.bits.len()).rev() {
                    let mut this = if self.bits[idx] { 1 } else { 0 };
                    let other = if sub[idx] { 1 } else { 0 } + overflow;
                    if other > this {
                        overflow = 1;
                        this += 2;
                    }
                    values.push(if this - other == 1 { true } else { false });
                }
                while let Some(false) = values.last() {
                    values.pop();
                }
                values.reverse();
                BigUInt { bits: values }
            }
        }
    }

    pub fn to_binary_string(&self) -> String {
        let mut res = String::new();
        if self.is_zero() {
            res.push('0');
        } else {
            for digit in &self.bits {
                if *digit {
                    res.push('1');
                } else {
                    res.push('0');
                }
            }
        }
        res
    }

    pub fn to_hex_string(&self) -> String {
        let mut res = String::new();
        if self.is_zero() {
            res.push('0');
        } else {
            let offset = self.bits.len() - 1;
            eprintln!("to_hex_string: <offset {}", offset);

            let mut values = Vec::new();
            let mut hex_digit = 0;
            for (idx, digit) in self.bits.iter().rev().enumerate() {
                if (idx > 0) && (idx % 4 == 0) {
                    values.push(HEX_DIGITS[hex_digit]);
                    hex_digit = 0;
                }
                if *digit {
                    hex_digit += 1 << idx % 4;
                }
            }

            values.push(HEX_DIGITS[hex_digit]);

            for digit in values.iter().rev() {
                res.push(*digit);
            }
        }
        res
    }
/*
    pub fn to_dec_string(&self) -> String {
        let mut res = String::new();
        let mut values = Vec::new();
        let mut dec_digit = 0;
        for (idx, digit) in self.big_int.iter().rev().enumerate() {

        }

        res

    }
*/
}




#[cfg(test)]
mod tests {
    use crate::BigUInt;

    #[test]
    fn test_from_u32() {
        let test = 0b11010101;
        let big_int = BigUInt::from_u32(test);
        assert_eq!(big_int.to_binary_string(),"11010101")
    }

    #[test]
    fn test_from_u64() {
        let test = 0b11010101u64;
        let big_int = BigUInt::from_u64(test);
        assert_eq!(big_int.to_binary_string(),"11010101");
        let test = 0xf0f0f0f0f0f0f0f0u64;
        let big_int = BigUInt::from_u64(test);
        assert_eq!(big_int.to_binary_string(),"1111000011110000111100001111000011110000111100001111000011110000")

    }

    #[test]
    fn test_to_hex_string() {
        let test = 0x1234ABCD;
        let big_int = BigUInt::from_u32(test);
        assert_eq!(big_int.to_hex_string(),"1234ABCD");
    }

    #[test]
    fn test_subtract() {
        let bi1 = BigUInt::from_u32(0b1000);
        let bi2 = BigUInt::from_u32(0b111);
        assert_eq!(bi1.subtract(bi2).to_binary_string(), "1");

        let bi1 = BigUInt::from_u32(0b1111);
        let bi2 = BigUInt::from_u32(0b1110);
        assert_eq!(bi1.subtract(bi2).to_binary_string(), "1");

        let bi1 = BigUInt::from_u32(0b1110);
        let bi2 = BigUInt::from_u32(0b1100);
        assert_eq!(bi1.subtract(bi2).to_binary_string(), "10");

    }

    #[test]
    #[should_panic]
    fn test_subtract_fail() {
        let bi1 = BigUInt::from_u32(0b1000);
        let bi2 = BigUInt::from_u32(0b111);

        bi2.subtract(bi1);
    }

    #[test]
    fn test_add() {
        let bi1 = BigUInt::from_u32(0b1);
        let bi2 = BigUInt::from_u32(0b1);
        assert_eq!(bi1.add(bi2).to_binary_string(), "10");

        let bi1 = BigUInt::from_u32(0b1010);
        let bi2 = BigUInt::from_u32(0b110);
        assert_eq!(bi1.add(bi2).to_binary_string(), "10000");

        let bi1 = BigUInt::from_u32(0b1010);
        let bi2 = BigUInt::from_u32(0b10);
        assert_eq!(bi1.add(bi2).to_binary_string(), "1100");

    }
}
