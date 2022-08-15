use std::fmt;
use std::ops::{Add, Mul, Neg, Shl, Shr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SvPrimaryLiteralIntegral {
    pub data_01: Vec<usize>,
    pub data_xz: Option<Vec<usize>>,
    pub size: usize,
    pub signed: bool,
}

// The following functions should be replaced by the build in methods once they become stable
impl SvPrimaryLiteralIntegral {
    /* Unsigned addition between two integral primary literals.
    Both data_01 vector dimensions (i.e nu of elements) are matched.
    It can be used for "signed" and "unsigned" values, and therefore the final number of bits is not derived within the function.
    Instead it must be explicitly implemented according the context that the function is used. */
    pub fn _unsigned_primlit_add(&mut self, mut right_nu: SvPrimaryLiteralIntegral) {
        self._primlit_vec_elmnt_match(&mut right_nu);

        let mut carry_flag: bool = false;

        for x in (0..self.data_01.len()).rev() {
            let left_nu: usize = self.data_01[x];
            self.data_01[x] = left_nu.wrapping_add(right_nu.data_01[x]);

            if carry_flag {
                self.data_01[x] = self.data_01[x].wrapping_add(1);
            }

            if self.data_01[x] >= left_nu && self.data_01[x] >= right_nu.data_01[x] {
                carry_flag = false;
            } else {
                carry_flag = true;
            }
        }

        if carry_flag {
            self.data_01.insert(0, 1);
        }
    }

    /* Accepts two integral primary literals and ensures that both data_01 vector dimensions (i.e nu of elements) are matched. */
    pub fn _primlit_vec_elmnt_match(&mut self, right_nu: &mut SvPrimaryLiteralIntegral) {
        let left_size = self.data_01.len();
        let right_size = right_nu.data_01.len();

        if left_size > right_size {
            let diff: usize = left_size - right_size;

            for _x in 0..diff {
                right_nu.data_01.insert(0, 0);
            }
        } else if left_size < right_size {
            let diff: usize = right_size - left_size;

            for _x in 0..diff {
                self.data_01.insert(0, 0);
            }
        }

        if self.is_4state() {
            self.data_xz = self.to_4state().data_xz;
        }

        if right_nu.is_4state() {
            right_nu.data_xz = right_nu.to_4state().data_xz;
        }
    }

    /* Receives a signed integral primary literal as an argument and deduces whether the stored value is -ve or +ve based on the size value set. */
    pub fn is_negative(&mut self) -> bool {
        if self.signed != true {
            panic!("Expected signed SvPrimaryLiteralIntegral but found unsigned!");
        }
        let zero = SvPrimaryLiteralIntegral {
            data_01: vec![0],
            data_xz: None,
            size: 1,
            signed: true,
        };

        let ret = self.lt(zero.clone());

        ret
    }

    /* Receives an integral primary literal as an argument and deduces whether the stored value is zero. */
    pub fn is_zero(&mut self) -> bool {
        let zero = SvPrimaryLiteralIntegral {
            data_01: vec![0],
            data_xz: None,
            size: 1,
            signed: true,
        };

        let one = SvPrimaryLiteralIntegral {
            data_01: vec![1],
            data_xz: None,
            size: 1,
            signed: false,
        };

        let ret = self.case_eq(zero.clone());

        ret == one
    }

    pub fn is_4state(&self) -> bool {
        match self.data_xz.clone() {
            None => false,
            Some(_) => true,
        }
    }

    /* Receives an integral primary literal as an argument and deduces whether it contains X(s) or Z(s). */
    pub fn contains_xz(&self) -> bool {
        if !self.is_4state() {
            return false;
        } else {
            for x in self.data_xz.as_ref().unwrap() {
                if x.leading_zeros() != usize::BITS {
                    return true;
                }
            }
        }

        false
    }

    /* Receives an integral primary literal and returns its contents in a 4-state integral primary literal. */
    pub fn to_4state(&self) -> SvPrimaryLiteralIntegral {
        let mut ret = SvPrimaryLiteralIntegral {
            data_01: self.data_01.clone(),
            data_xz: Some(vec![0]),
            size: self.size,
            signed: self.signed,
        };

        if ret.data_01.len() != ret.data_xz.as_ref().unwrap().len() {
            for _x in 0..(ret.data_01.len() - ret.data_xz.as_ref().unwrap().len()) {
                let mut new_vec = ret.data_xz.clone().unwrap();
                new_vec.insert(0, 0);
                ret.data_xz = Some(new_vec);
            }
        }

        ret
    }

    /* Accepts two signed integral primary literals and ensures that both are properly sign extended and matched to their data_01 dimensions.
    The correct final number of bits is set to both arguments. */
    pub fn _matched_sign_extend(&mut self, right_nu: &mut SvPrimaryLiteralIntegral) {
        if self.signed != true || right_nu.signed != true {
            panic!("Expected signed SvPrimaryLiterals but found unsigned!");
        }

        let left_neg: bool = self.is_negative();
        let right_neg: bool = right_nu.is_negative();

        self._primlit_vec_elmnt_match(right_nu);

        if left_neg {
            let mut last_element: bool = false;

            for x in 0..self.data_01.len() {
                let left_leading = self.data_01[x].leading_zeros();

                if left_leading != usize::BITS {
                    last_element = true;
                }

                for y in 0..left_leading {
                    self.data_01[x] = self.data_01[x] + 2usize.pow(usize::BITS - y - 1);
                }

                if last_element {
                    break;
                }
            }
        }

        if right_neg {
            let mut last_element: bool = false;

            for x in 0..right_nu.data_01.len() {
                let left_leading = right_nu.data_01[x].leading_zeros();

                if left_leading != usize::BITS {
                    last_element = true;
                }

                for y in 0..left_leading {
                    right_nu.data_01[x] = right_nu.data_01[x] + 2usize.pow(usize::BITS - y - 1);
                }

                if last_element {
                    break;
                }
            }
        }

        self.size = self.data_01.len() * usize::BITS as usize;
        right_nu.size = right_nu.data_01.len() * usize::BITS as usize;
    }

    /* Receives a signed integral primary literal and sign extends the value in the existing number of data_01 vector elements.
    The correct final number of bits is set to the argument. */
    pub fn _sign_extend(&mut self) {
        if self.signed != true {
            panic!("Expected signed SvPrimaryLiteralIntegral but found unsigned!");
        }

        let left_neg: bool = self.is_negative();

        if left_neg {
            let mut last_element: bool = false;

            for x in 0..self.data_01.len() {
                let left_leading = self.data_01[x].leading_zeros();

                if left_leading != usize::BITS {
                    last_element = true;
                }

                for y in 0..left_leading {
                    self.data_01[x] = self.data_01[x] + 2usize.pow(usize::BITS - y - 1);
                }

                if last_element {
                    break;
                }
            }
        }

        self.size = self.data_01.len() * usize::BITS as usize;
    }

    /* Receives a signed integral primary literal and returns its opposite signed primary literal (i.e +ve -> -ve and vice versa).
    The correct final number of bits is set to the argument. */
    pub fn nega(&self) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();
        if ret.is_zero() {
            return ret;
        } else if ret.signed != true {
            panic!("Expected signed SvPrimaryLiteralIntegral but found unsigned!");
        }

        let from_negative: bool = ret.is_negative();
        ret._sign_extend();

        ret = ret.inv();

        ret = ret + 1;

        if from_negative {
            ret.size = (usize::BITS as usize - ret.data_01[0].leading_zeros() as usize + 1)
                + (ret.data_01.len() - 1) * usize::BITS as usize;

            if ret.data_01[0].leading_zeros() == 0 {
                ret.data_01.insert(0, 0);
            }
        } else {
            ret.size = (usize::BITS as usize - ret.data_01[0].leading_zeros() as usize)
                + (ret.data_01.len() - 1) * usize::BITS as usize;
            ret._minimum_width();
        }

        if ret.is_4state() {
            ret.data_xz = ret.to_4state().data_xz;
        }

        ret
    }

    /* Receives a signed integral primary literal and returns a primary literal with its inverted value.
    The final number of bits remains the same as the original one.*/
    pub fn inv(&self) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();

        let first_elmnt_bits: u32;
        if ret.size % usize::BITS as usize == 0 {
            first_elmnt_bits = usize::BITS;
        } else {
            first_elmnt_bits = ret.size as u32 % usize::BITS;
        }
        let remaining_bits = usize::BITS - first_elmnt_bits;

        for _x in 0..ret.size {
            if ret.is_4state()
                && (ret.data_xz.as_ref().unwrap()[0].leading_zeros() == remaining_bits)
            {
                if ret.data_01[0].leading_zeros() == remaining_bits {
                    ret.data_01[0] = ret.data_01[0] - 2usize.pow(first_elmnt_bits - 1);
                }
            } else if ret.data_01[0].leading_zeros() == remaining_bits {
                ret.data_01[0] = ret.data_01[0] - 2usize.pow(first_elmnt_bits - 1);
            } else {
                ret.data_01[0] = ret.data_01[0] + 2usize.pow(first_elmnt_bits - 1);
            }

            ret = ret.ror(1);
        }

        ret
    }

    /* Receives the number of shift positions and implements logical shifting to the left.
    For each shift the total number of bits increments by 1 i.e. lsl works as 2^(positions) and the size of the integral primlit is dynamically adjusted.
    If an explicit range is defined, _truncate can be used afterwards.*/
    pub fn lsl(&self, n: usize) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();

        for _x in 0..n {
            let mut leading_one: bool = false;
            let mut leading_one_xz: bool = false;

            ret.size = ret.size + 1;

            for y in (0..ret.data_01.len()).rev() {
                let pre_mod = ret.data_01[y];

                if leading_one {
                    ret.data_01[y] = (ret.data_01[y] << 1) + 1;
                    leading_one = false;
                } else {
                    ret.data_01[y] = ret.data_01[y] << 1;
                }

                if pre_mod.leading_zeros() == 0 {
                    leading_one = true;
                }

                if ret.is_4state() {
                    let pre_mod = ret.data_xz.as_ref().unwrap()[y];

                    if leading_one_xz {
                        ret.data_xz.as_mut().unwrap()[y] =
                            (ret.data_xz.as_ref().unwrap()[y] << 1) + 1;
                        leading_one_xz = false;
                    } else {
                        ret.data_xz.as_mut().unwrap()[y] = ret.data_xz.as_ref().unwrap()[y] << 1;
                    }

                    if pre_mod.leading_zeros() == 0 {
                        leading_one_xz = true;
                    }
                }
            }

            if leading_one && leading_one_xz {
                ret.data_01.insert(0, 1);
                ret.data_xz.as_mut().unwrap().insert(0, 1);
            } else if leading_one {
                ret.data_01.insert(0, 1);
                if ret.is_4state() {
                    ret.data_xz.as_mut().unwrap().insert(0, 0);
                }
            } else if leading_one_xz {
                ret.data_01.insert(0, 0);
                ret.data_xz.as_mut().unwrap().insert(0, 1);
            } else if ret.signed && (ret.size > usize::BITS as usize * ret.data_01.len()) {
                ret.data_01.insert(0, 0);

                if ret.is_4state() {
                    ret.data_xz.as_mut().unwrap().insert(0, 0);
                }
            }
        }

        ret
    }

    /* Receives the number of shift positions and implements logical shifting to the right.
    The initial number of bits is preserved. */
    pub fn lsr(&self, n: usize) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();

        for _x in 0..n {
            let mut trailing_one: bool = false;
            let mut trailing_one_xz: bool = false;

            for y in 0..ret.data_01.len() {
                let pre_mod = ret.data_01[y];

                if trailing_one {
                    ret.data_01[y] = (ret.data_01[y] >> 1) + 2usize.pow(usize::BITS - 1);
                    trailing_one = false;
                } else {
                    ret.data_01[y] = ret.data_01[y] >> 1;
                }

                if pre_mod.trailing_zeros() == 0 {
                    trailing_one = true;
                }

                if ret.is_4state() {
                    let pre_mod = ret.data_xz.as_ref().unwrap()[y];

                    if trailing_one_xz {
                        ret.data_xz.as_mut().unwrap()[y] =
                            (ret.data_xz.as_ref().unwrap()[y] >> 1) + 2usize.pow(usize::BITS - 1);
                        trailing_one_xz = false;
                    } else {
                        ret.data_xz.as_mut().unwrap()[y] = ret.data_xz.as_ref().unwrap()[y] >> 1;
                    }

                    if pre_mod.trailing_zeros() == 0 {
                        trailing_one_xz = true;
                    }
                }
            }
        }

        ret
    }

    /* Receives the number of shift positions and shifts the value to the left without changing the number of bits.
    The dropped bits are shifted in the RHS of the value. */
    pub fn rol(&self, n: usize) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();
        let last_index = ret.data_01.len() - 1;

        for _x in 0..n {
            let previous_size = ret.size;
            let leading_one: bool;
            let mut leading_one_xz: bool = false;

            if previous_size % usize::BITS as usize == 0 {
                leading_one = ret.data_01[0].leading_zeros() == 0;

                if ret.is_4state() {
                    leading_one_xz = ret.data_xz.as_ref().unwrap()[0].leading_zeros() == 0;
                }
            } else {
                leading_one = ret.data_01[0].leading_zeros() as usize
                    == (usize::BITS as usize - (ret.size % usize::BITS as usize));

                if ret.is_4state() {
                    leading_one_xz = ret.data_xz.as_ref().unwrap()[0].leading_zeros() as usize
                        == (usize::BITS as usize - (ret.size % usize::BITS as usize));
                }
            }

            ret = ret.lsl(1);
            ret._truncate(previous_size);
            if leading_one {
                ret.data_01[last_index] = ret.data_01[last_index] + 1;
            }

            if leading_one_xz {
                ret.data_xz.as_mut().unwrap()[last_index] =
                    ret.data_xz.as_ref().unwrap()[last_index] + 1;
            }
        }

        ret
    }

    /* Receives the number of shift positions and shifts the value to the right without changing the number of bits.
    The dropped bits are shifted in the LHS of the value. */
    pub fn ror(&self, n: usize) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();
        let last_index = ret.data_01.len() - 1;
        let msb: u32;

        if ret.size % usize::BITS as usize == 0 {
            msb = usize::BITS;
        } else {
            msb = ret.size as u32 % usize::BITS;
        }

        for _x in 0..n {
            let trailing_one: bool = ret.data_01[last_index].trailing_zeros() == 0;
            let mut trailing_one_xz: bool = false;

            if ret.is_4state() {
                trailing_one_xz = ret.data_xz.as_ref().unwrap()[last_index].trailing_zeros() == 0;
            }

            ret = ret.lsr(1);

            if trailing_one {
                ret.data_01[0] = ret.data_01[0] + 2usize.pow(msb - 1);
            }

            if trailing_one_xz {
                ret.data_xz.as_mut().unwrap()[0] =
                    ret.data_xz.as_ref().unwrap()[0] + 2usize.pow(msb - 1);
            }
        }

        ret
    }

    /* Receives two integral primary literals, concatenates them (logically shifts left the LHS primlit by RHS primlit's size and adds them).
    Returns an integral SvPrimaryLiteralIntegral with the final value. */
    pub fn cat(&self, right_nu: SvPrimaryLiteralIntegral) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();
        ret = ret.lsl(right_nu.size);

        let mut left_nu: SvPrimaryLiteralIntegral = ret.clone();

        if left_nu.is_4state() || right_nu.is_4state() {
            let mut left_xz = SvPrimaryLiteralIntegral {
                data_01: left_nu.data_xz.as_ref().unwrap().clone(),
                data_xz: None,
                size: left_nu.size,
                signed: false,
            };

            let right_xz = SvPrimaryLiteralIntegral {
                data_01: right_nu.data_xz.as_ref().unwrap().clone(),
                data_xz: None,
                size: right_nu.size,
                signed: false,
            };

            left_xz._unsigned_primlit_add(right_xz.clone());
            left_nu.data_xz = Some(left_xz.data_01.clone());
        }

        ret._unsigned_primlit_add(right_nu.clone());
        ret.size = self.size + right_nu.size;
        ret.data_xz = left_nu.data_xz.clone();

        ret
    }

    /* Compares two signed or unsigned integral primary literals and if the value of the RHS primlit is greater than the LHS it returns true.
    Otherwise it returns false. By default if any of the two operands contains X/Z the comparison returns false. */
    pub fn lt(&self, mut right_nu: SvPrimaryLiteralIntegral) -> bool {
        let mut left_nu = self.clone();

        if left_nu.contains_xz() || right_nu.contains_xz() {
            return false;
        } else if left_nu.signed != right_nu.signed {
            left_nu.signed = false;
            right_nu.signed = false;

            return left_nu.lt(right_nu.clone());
        } else {
            if left_nu.signed {
                let left_nu_neg: bool;
                let right_nu_neg: bool;

                let left_leading_zeros: usize = usize::BITS as usize
                    - (left_nu.size - (left_nu.data_01.len() - 1) * usize::BITS as usize);

                if left_nu.data_01[0].leading_zeros() as usize == left_leading_zeros {
                    left_nu_neg = true;
                } else {
                    left_nu_neg = false;
                }

                let right_leading_zeros: usize = usize::BITS as usize
                    - (right_nu.size - (right_nu.data_01.len() - 1) * usize::BITS as usize);

                if right_nu.data_01[0].leading_zeros() as usize == right_leading_zeros {
                    right_nu_neg = true;
                } else {
                    right_nu_neg = false;
                }

                if left_nu_neg && !right_nu_neg {
                    return true;
                } else if !left_nu_neg && right_nu_neg {
                    return false;
                } else {
                    if left_nu_neg {
                        left_nu._minimum_width();
                        right_nu._minimum_width();

                        if left_nu.size > right_nu.size {
                            return true;
                        }
                    } else {
                        left_nu.signed = false;
                        right_nu.signed = false;

                        left_nu._minimum_width();
                        right_nu._minimum_width();

                        if left_nu.size < right_nu.size {
                            return true;
                        }
                    }
                }
            } else {
                left_nu._minimum_width();
                right_nu._minimum_width();

                if left_nu.size < right_nu.size {
                    return true;
                }
            }

            false
        }
    }

    /* Compares two signed or unsigned integral primary literals and if the value of the LHS primlit is greater than the RHS it returns true.
    Otherwise it returns false. By default if any of the two operands contains X/Z the comparison returns false. */
    pub fn gt(&self, mut right_nu: SvPrimaryLiteralIntegral) -> bool {
        let mut left_nu = self.clone();

        if left_nu.contains_xz() || right_nu.contains_xz() {
            return false;
        } else if left_nu.signed != right_nu.signed {
            left_nu.signed = false;
            right_nu.signed = false;

            return left_nu.gt(right_nu.clone());
        } else {
            if left_nu.signed {
                let left_nu_neg: bool;
                let right_nu_neg: bool;

                let left_leading_zeros: usize = usize::BITS as usize
                    - (left_nu.size - (left_nu.data_01.len() - 1) * usize::BITS as usize);

                if left_nu.data_01[0].leading_zeros() as usize == left_leading_zeros {
                    left_nu_neg = true;
                } else {
                    left_nu_neg = false;
                }

                let right_leading_zeros: usize = usize::BITS as usize
                    - (right_nu.size - (right_nu.data_01.len() - 1) * usize::BITS as usize);

                if right_nu.data_01[0].leading_zeros() as usize == right_leading_zeros {
                    right_nu_neg = true;
                } else {
                    right_nu_neg = false;
                }

                if left_nu_neg && !right_nu_neg {
                    return false;
                } else if !left_nu_neg && right_nu_neg {
                    return true;
                } else {
                    if left_nu_neg {
                        left_nu._minimum_width();
                        right_nu._minimum_width();

                        if left_nu.size < right_nu.size {
                            return true;
                        }
                    } else {
                        left_nu.signed = false;
                        right_nu.signed = false;

                        left_nu._minimum_width();
                        right_nu._minimum_width();

                        if left_nu.size > right_nu.size {
                            return true;
                        }
                    }
                }
            } else {
                left_nu._minimum_width();
                right_nu._minimum_width();

                if left_nu.size > right_nu.size {
                    return true;
                }
            }

            false
        }
    }

    /* Compares two signed or unsigned integral primary literals and if the value of the LHS primlit is equal to the RHS it returns true.
    Otherwise it returns false. */
    pub fn case_eq(&self, mut right_nu: SvPrimaryLiteralIntegral) -> SvPrimaryLiteralIntegral {
        let mut left_nu = self.clone();
        let zero = SvPrimaryLiteralIntegral {
            data_01: vec![0],
            data_xz: None,
            size: 1,
            signed: false,
        };
        let one = SvPrimaryLiteralIntegral {
            data_01: vec![1],
            data_xz: None,
            size: 1,
            signed: false,
        };

        if left_nu.contains_xz() != right_nu.contains_xz() {
            return zero;
        } else if left_nu.contains_xz() && right_nu.contains_xz() {
            let signedness = left_nu.signed == right_nu.signed;
            let size = left_nu.size == right_nu.size;
            let data_01 = left_nu.data_01 == right_nu.data_01;
            let data_xz = left_nu.data_xz.as_ref().unwrap() == right_nu.data_xz.as_ref().unwrap();

            if signedness && size && data_01 && data_xz {
                return one;
            }
        } else if left_nu.signed != right_nu.signed {
            left_nu.signed = false;
            right_nu.signed = false;

            return left_nu.case_eq(right_nu.clone());
        } else {
            let mut left_zero: bool = true;
            for x in &left_nu.data_01 {
                if x.leading_zeros() != usize::BITS {
                    left_zero = false;
                }
            }

            let mut right_zero: bool = true;
            for y in &right_nu.data_01 {
                if y.leading_zeros() != usize::BITS {
                    right_zero = false;
                }
            }

            if left_zero && right_zero {
                return one;
            } else if left_zero != right_zero {
                return zero;
            } else {
                left_nu._minimum_width();
                right_nu._minimum_width();

                if left_nu.size == right_nu.size {
                    let mut eq_found: bool = true;
                    for x in 0..left_nu.data_01.len() {
                        if left_nu.data_01[x] != right_nu.data_01[x] {
                            eq_found = false;
                        }
                    }
                    if eq_found {
                        return one;
                    }
                }
            }
        }

        zero
    }

    /* Receives a signed or unsigned integral primary literal and deduces an equivalent representation with the minimum number of bits required.
    The correct final number of bits is set to the argument. */
    pub fn _minimum_width(&mut self) {
        if !self.signed {
            if self.is_zero() {
                for _x in 0..self.data_01.len() {
                    self.data_01.remove(0);
                }
                self.data_01.push(0);
                self.size = 1;
            } else {
                for _x in 0..self.data_01.len() {
                    if self.data_01[0] == 0 {
                        self.data_01.remove(0);
                    }
                }

                self.size = (usize::BITS as usize - self.data_01[0].leading_zeros() as usize)
                    + (self.data_01.len() - 1) * usize::BITS as usize;
            }
        } else {
            let mut min_num_found: bool = false;
            let mut vec_elements_to_rm: usize = 0;

            if self.is_negative() {
                for x in 0..self.data_01.len() {
                    while !min_num_found {
                        let pre_leading = self.data_01[x].leading_zeros();

                        let minimized_value: usize =
                            self.data_01[x] - 2usize.pow(usize::BITS - pre_leading - 1); //TODO
                        let post_leading = minimized_value.leading_zeros();

                        if post_leading == usize::BITS {
                            if x == (self.data_01.len() - 1)
                                || self.data_01[x + 1].leading_zeros() != 0
                            {
                                min_num_found = true;
                                break;
                            }
                        }

                        if post_leading != (pre_leading + 1) {
                            min_num_found = true;
                            break;
                        } else {
                            self.data_01[x] = minimized_value;
                            self.size = self.size - 1;

                            if post_leading == usize::BITS {
                                vec_elements_to_rm = vec_elements_to_rm + 1;
                                break;
                            }
                        }
                    }
                }

                for _x in 0..vec_elements_to_rm {
                    self.data_01.remove(0);
                }
            } else if self.is_zero() {
                for _x in 0..self.data_01.len() {
                    self.data_01.remove(0);
                }
                self.data_01.push(0);
                self.size = 1;
            } else {
                for _x in 0..self.data_01.len() {
                    if self.data_01[0] == 0 {
                        self.data_01.remove(0);
                    }
                }

                if self.data_01[0].leading_zeros() == 0 {
                    self.data_01.insert(0, 0);
                }

                self.size = (usize::BITS as usize - self.data_01[0].leading_zeros() as usize + 1)
                    + (self.data_01.len() - 1) * usize::BITS as usize;
            }
        }

        if self.is_4state() && (self.data_01.len() < self.data_xz.as_ref().unwrap().len()) {
            self.data_xz = self.to_4state().data_xz;
        }
    }

    /* Receives the number of bits in which an integral primary literal should be truncated.
    The correct final number of bits is set but the signedness doesn't change. */
    pub fn _truncate(&mut self, size: usize) {
        if size == 0 {
            panic!("Cannot truncate the value to zero bits!");
        } else if self.size >= size {
            let elmnts_to_be_rm: usize;
            let bits_to_be_rm: usize;

            if (size % usize::BITS as usize) == 0 {
                elmnts_to_be_rm = self.data_01.len() - size / usize::BITS as usize;
                bits_to_be_rm = 0;
            } else {
                elmnts_to_be_rm = self.data_01.len() - (size / usize::BITS as usize) - 1;
                bits_to_be_rm = usize::BITS as usize - size % usize::BITS as usize;
            }

            for _x in 0..elmnts_to_be_rm {
                self.data_01.remove(0);
            }

            if bits_to_be_rm != 0 {
                for x in
                    ((usize::BITS as usize - bits_to_be_rm + 1)..(usize::BITS as usize + 1)).rev()
                {
                    if self.data_01[0].leading_zeros() == (usize::BITS - x as u32) {
                        self.data_01[0] = self.data_01[0] - 2usize.pow(x as u32 - 1);
                    }
                }
            }

            if self.is_4state() {
                let elmnts_to_be_rm: usize;
                let bits_to_be_rm: usize;

                if (size % usize::BITS as usize) == 0 {
                    elmnts_to_be_rm =
                        self.data_xz.as_ref().unwrap().len() - size / usize::BITS as usize;
                    bits_to_be_rm = 0;
                } else {
                    elmnts_to_be_rm =
                        self.data_xz.as_ref().unwrap().len() - (size / usize::BITS as usize) - 1;
                    bits_to_be_rm = usize::BITS as usize - size % usize::BITS as usize;
                }

                for _x in 0..elmnts_to_be_rm {
                    self.data_xz.as_mut().unwrap().remove(0);
                }

                if bits_to_be_rm != 0 {
                    for x in ((usize::BITS as usize - bits_to_be_rm + 1)
                        ..(usize::BITS as usize + 1))
                        .rev()
                    {
                        if self.data_xz.as_ref().unwrap()[0].leading_zeros()
                            == (usize::BITS - x as u32)
                        {
                            self.data_xz.as_mut().unwrap()[0] =
                                self.data_xz.as_ref().unwrap()[0] - 2usize.pow(x as u32 - 1);
                        }
                    }
                }
            }

            self.size = size;
        } else {
            panic!("The original number of bits is smaller than the requested one!");
        }
    }

    pub fn add_primlit(&self, mut right_nu: SvPrimaryLiteralIntegral) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();

        if ret.is_4state() != right_nu.is_4state() {
            if !ret.is_4state() {
                ret = ret.to_4state();
            } else {
                right_nu = right_nu.to_4state();
            }
        }

        if !ret.contains_xz() && !right_nu.contains_xz() {
            if ret.signed == false || right_nu.signed == false {
                ret.signed = false;
                ret._unsigned_primlit_add(right_nu.clone());
                ret._minimum_width();
            } else {
                let left_neg: bool = ret.is_negative();
                let right_neg: bool = right_nu.is_negative();

                if !left_neg && !right_neg {
                    let new_size: usize;

                    ret._unsigned_primlit_add(right_nu.clone());

                    if ret.data_01[0].leading_zeros() == 0 {
                        ret.data_01.insert(0, 0);
                    }

                    new_size = (usize::BITS as usize - ret.data_01[0].leading_zeros() as usize + 1)
                        + (ret.data_01.len() - 1) * usize::BITS as usize;

                    ret.size = new_size;
                } else if left_neg && right_neg {
                    let new_size: usize;

                    ret._matched_sign_extend(&mut right_nu);
                    ret._unsigned_primlit_add(right_nu.clone());

                    new_size = (usize::BITS as usize - ret.data_01[0].leading_zeros() as usize)
                        + (ret.data_01.len() - 1) * usize::BITS as usize;
                    ret.size = new_size;

                    ret._minimum_width();
                } else {
                    ret._matched_sign_extend(&mut right_nu);
                    ret._unsigned_primlit_add(right_nu.clone());
                    ret._truncate(ret.size);

                    ret._minimum_width();
                }
            }

            if ret.is_4state() {
                ret.data_xz = ret.to_4state().data_xz;
            }

            ret
        } else {
            if ret.size < right_nu.size {
                ret.size = right_nu.size;
            }

            // Possible carry out from the MSB
            let final_num_bits = ret.size + 1;

            ret = SvPrimaryLiteralIntegral {
                data_01: vec![0],
                data_xz: Some(vec![1]),
                signed: !(ret.signed == false || right_nu.signed == false),
                size: 1,
            };

            let x_primlit = SvPrimaryLiteralIntegral {
                data_01: vec![0],
                data_xz: Some(vec![1]),
                signed: ret.signed,
                size: 1,
            };

            for _x in 0..(final_num_bits - 1) {
                ret = ret.cat(x_primlit.clone());
            }

            ret
        }
    }

    pub fn mul_unsigned(&self, mut right_nu: SvPrimaryLiteralIntegral) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral;
        let mut left_nu: SvPrimaryLiteralIntegral = self.clone();
        let mut add_ver: Vec<SvPrimaryLiteralIntegral> = Vec::new();
        let last_index = right_nu.data_01.len() - 1;

        for x in 0..right_nu.size {
            if right_nu.data_01[last_index].trailing_zeros() == 0 {
                if x == 0 {
                    add_ver.push(left_nu.clone());
                } else {
                    left_nu = left_nu.lsl(1);
                    add_ver.push(left_nu.clone());
                }
            } else if x != 0 {
                left_nu = left_nu.lsl(1);
            }

            right_nu = right_nu.lsr(1);
        }
        ret = SvPrimaryLiteralIntegral {
            data_01: vec![0],
            data_xz: None,
            signed: false,
            size: 1,
        };

        for y in 0..add_ver.len() {
            ret = ret.add_primlit(add_ver[y].clone());
        }

        ret
    }

    pub fn mult(&self, mut right_nu: SvPrimaryLiteralIntegral) -> SvPrimaryLiteralIntegral {
        let mut left_nu: SvPrimaryLiteralIntegral = self.clone();
        let mut ret: SvPrimaryLiteralIntegral;

        if left_nu.is_4state() != right_nu.is_4state() {
            if !left_nu.is_4state() {
                left_nu = left_nu.to_4state();
            } else {
                right_nu = right_nu.to_4state();
            }
        }

        if !left_nu.contains_xz() && !right_nu.contains_xz() {
            if !left_nu.signed || !right_nu.signed {
                left_nu.signed = false;
                right_nu.signed = false;

                left_nu._minimum_width();
                right_nu._minimum_width();

                ret = left_nu.mul_unsigned(right_nu.clone());
                ret._minimum_width();
            } else {
                let left_neg: bool = left_nu.is_negative();
                let right_neg: bool = right_nu.is_negative();
                let result_neg: bool;

                if left_neg && right_neg {
                    left_nu = left_nu.neg();
                    right_nu = right_nu.neg();
                    result_neg = false;
                } else if left_neg || right_neg {
                    if left_neg {
                        left_nu = left_nu.neg();
                    } else {
                        right_nu = right_nu.neg();
                    }
                    result_neg = true;
                } else {
                    result_neg = false;
                }

                left_nu.signed = false;
                right_nu.signed = false;

                left_nu._minimum_width();
                right_nu._minimum_width();

                ret = left_nu.mul_unsigned(right_nu.clone());
                ret._minimum_width();
                ret.signed = true;

                if result_neg {
                    ret.size = ret.size + 1;
                    ret = ret.neg();
                } else {
                    ret.size = ret.size + 1;
                }
            }

            if ret.is_4state() {
                ret.data_xz = ret.to_4state().data_xz;
            }
        } else {
            let final_num_bits = left_nu.size + right_nu.size;

            ret = SvPrimaryLiteralIntegral {
                data_01: vec![0],
                data_xz: Some(vec![1]),
                signed: !(left_nu.signed == false || right_nu.signed == false),
                size: 1,
            };

            let x_primlit = SvPrimaryLiteralIntegral {
                data_01: vec![0],
                data_xz: Some(vec![1]),
                signed: ret.signed,
                size: 1,
            };

            for _x in 0..(final_num_bits - 1) {
                ret = ret.cat(x_primlit.clone());
            }
        }

        ret
    }
}

pub fn usize_to_primlit(value: usize) -> SvPrimaryLiteralIntegral {
    let mut ret = SvPrimaryLiteralIntegral {
        data_01: vec![value],
        data_xz: None,
        size: usize::BITS as usize,
        signed: true,
    };

    ret._minimum_width();

    ret
}

impl fmt::Display for SvPrimaryLiteralIntegral {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "NumBits: {}", self.size)?;
        writeln!(f, "Signed: {}", self.signed)?;
        writeln!(f, "4State: {}", self.is_4state())?;

        let mut string_vec: Vec<String> = Vec::new();
        let mut s: String = String::new();
        let mut mod_primlit = self.clone();
        let first_elmnt_bits: u32;

        if mod_primlit.size % usize::BITS as usize == 0 {
            first_elmnt_bits = usize::BITS;
        } else {
            first_elmnt_bits = mod_primlit.size as u32 % usize::BITS;
        }
        let remaining_bits = usize::BITS - first_elmnt_bits;

        for _x in 0..first_elmnt_bits {
            if mod_primlit.is_4state()
                && (mod_primlit.data_xz.as_ref().unwrap()[0].leading_zeros() == remaining_bits)
            {
                if mod_primlit.data_01[0].leading_zeros() == remaining_bits {
                    s.push('Z');
                } else {
                    s.push('X');
                }
            } else if mod_primlit.data_01[0].leading_zeros() == remaining_bits {
                s.push('1');
            } else {
                s.push('0');
            }

            mod_primlit = mod_primlit.rol(1);
        }

        string_vec.push(s);

        if self.data_01.len() > 1 {
            for x in 1..self.data_01.len() {
                let mut mod_primlit = self.clone();
                let mut s: String = String::new();

                for _y in 0..usize::BITS {
                    if mod_primlit.is_4state()
                        && (mod_primlit.data_xz.as_ref().unwrap()[x].leading_zeros() == 0)
                    {
                        if mod_primlit.data_01[x].leading_zeros() == 0 {
                            s.push('Z');
                        } else {
                            s.push('X');
                        }
                    } else if mod_primlit.data_01[x].leading_zeros() == 0 {
                        s.push('1');
                    } else {
                        s.push('0');
                    }

                    mod_primlit = mod_primlit.rol(1);
                }

                string_vec.push(s);
            }
        }

        write!(f, "Data: ")?;
        for x in string_vec {
            writeln!(f, "{}", x)?;
        }

        write!(f, "")
    }
}

impl Add for SvPrimaryLiteralIntegral {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.add_primlit(rhs.clone())
    }
}

impl Add<usize> for SvPrimaryLiteralIntegral {
    type Output = Self;

    fn add(self, rhs: usize) -> Self {
        let rhs = usize_to_primlit(rhs);
        self.add_primlit(rhs.clone())
    }
}

impl Mul for SvPrimaryLiteralIntegral {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.mult(rhs.clone())
    }
}

impl Shl<usize> for SvPrimaryLiteralIntegral {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self {
        self.lsl(rhs)
    }
}

impl Shr<usize> for SvPrimaryLiteralIntegral {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self {
        self.lsr(rhs)
    }
}

impl Neg for SvPrimaryLiteralIntegral {
    type Output = Self;

    fn neg(self) -> Self {
        if self.contains_xz() {
            panic!("Cannot negate an integral primary literal that contains X/Z!");
        }
        self.nega()
    }
}
