use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SvPrimaryLiteralIntegral {
    pub data_01: Vec<usize>,
    pub data_xz: Option<Vec<usize>>,
    pub size: usize,
    pub signed: bool,
}

// The following functions should be replaced by the build in methods once they become stable
impl SvPrimaryLiteralIntegral {
    /* Unsigned addition between an integral primary literal and usize.
    It can be used for "signed" and "unsigned" values, and therefore the final number of bits is not derived within the function.
    Instead it must be explicitly implemented according the context that the function is used. */
    pub fn _unsigned_usize_add(&mut self, right_nu: usize) {
        let last_index = self.data_01.len() - 1;
        let left_nu: usize = self.data_01[last_index];
        self.data_01[last_index] = left_nu.wrapping_add(right_nu.clone());

        if (self.data_01[last_index] < left_nu) || (self.data_01[last_index] < right_nu.clone()) {
            if self.data_01.len() == 1 {
                self.data_01.insert(0, 1);
            } else {
                let mut carry_flag: bool = true;

                for x in (0..self.data_01.len() - 1).rev() {
                    let left_nu: usize = self.data_01[x];
                    self.data_01[x] = left_nu.wrapping_add(1);

                    if self.data_01[x] > left_nu {
                        carry_flag = false;
                        break;
                    }
                }

                if carry_flag {
                    self.data_01.insert(0, 1);
                }
            }
        }
    }

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
    }

    /* Receives a signed integral primary literal as an argument and deduces whether the stored value is -ve or +ve based on the size value set. */
    pub fn is_negative(&mut self) -> bool {
        if self.signed != true {
            panic!("Expected signed SvPrimaryLiteralIntegral but found unsigned!");
        }

        let leading_zeros: usize =
            usize::BITS as usize - (self.size - (self.data_01.len() - 1) * usize::BITS as usize);

        if self.data_01[0].leading_zeros() as usize == leading_zeros {
            true
        } else {
            false
        }
    }

    /* Receives an integral primary literal as an argument and deduces whether the stored value is zero. */
    pub fn is_zero(&mut self) -> bool {
        for x in &self.data_01 {
            if x.leading_zeros() != usize::BITS {
                return false;
            }
        }

        true
    }

    pub fn is_4state(&self) -> bool {
        match self.data_xz.clone() {
            None => false,
            Some(_) => true,
        }
    }

    /* Receives an integral primary literal as an argument and deduces whether it contains X(s) or Z(s). */
    pub fn contains_nonbinary(&self) -> bool {
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

    /* Receives a 2-state integral primary literal and returns its conversion to a 4-state integral primary literal. */
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
    pub fn _matched_sign_extension(&mut self, right_nu: &mut SvPrimaryLiteralIntegral) {
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
    pub fn _sign_extension(&mut self) {
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
    pub fn neg(&self) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();
        if ret.is_zero() {
            return ret;
        } else if ret.signed != true {
            panic!("Expected signed SvPrimaryLiteralIntegral but found unsigned!");
        }

        let from_negative: bool = ret.is_negative();
        ret._sign_extension();

        ret = ret.inv();

        ret._unsigned_usize_add(1);

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

        ret
    }

    /* Receives a signed integral primary literal and returns a primary literal with its inverted value.
    The final number of bits remains the same as the original one.*/
    pub fn inv(&self) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();

        for x in 0..ret.data_01.len() {
            ret.data_01[x] = !ret.data_01[x];
        }

        ret._truncate(ret.size);

        ret
    }

    /* Receives the number of shift positions and implements logical shifting to the left.
    For each shift the total number of bits increments by 1 i.e. lsl works as 2^(positions) and the size of the integral primlit is dynamically adjusted.
    If an explicit range is defined, _truncate can be used afterwards.*/
    pub fn lsl(&self, n: usize) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();

        for _x in 0..n {
            let mut leading_one: bool = false;
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
            }

            if leading_one {
                ret.data_01.insert(0, 1);
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

            if previous_size % usize::BITS as usize == 0 {
                leading_one = ret.data_01[0].leading_zeros() == 0;
            } else {
                leading_one = ret.data_01[0].leading_zeros() as usize
                    == (usize::BITS as usize - (ret.size % usize::BITS as usize));
            }

            ret = ret.lsl(1);
            ret._truncate(previous_size);
            if leading_one {
                ret.data_01[last_index] = ret.data_01[last_index] + 1;
            }
        }

        ret
    }

    /* Receives the number of shift positions and shifts the value to the right without changing the number of bits.
    The dropped bits are shifted in the LHS of the value. */
    pub fn ror(&self, n: usize) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();
        let last_index = ret.data_01.len() - 1;

        for _x in 0..n {
            let trailing_one: bool = ret.data_01[last_index].trailing_zeros() == 0;
            ret = ret.lsr(1);

            if trailing_one {
                ret.data_01[0] = ret.data_01[0] + 2usize.pow(usize::BITS - 1);
            }
        }

        ret
    }

    /* Receives two integral primary literals, concatenates them (logically shifts left the LHS primlit by RHS primlit's size and adds them).
    Returns an integral SvPrimaryLiteralIntegral with the final value. */
    pub fn cat(&self, right_nu: SvPrimaryLiteralIntegral) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();
        ret = ret.lsl(right_nu.size);
        ret._unsigned_primlit_add(right_nu.clone());
        ret.size = self.size + right_nu.size;

        ret
    }

    /* Compares two signed or unsigned integral primary literals and if the value of the RHS primlit is greater than the LHS it returns true.
    Otherwise it returns false. */
    pub fn lt(&self, mut right_nu: SvPrimaryLiteralIntegral) -> bool {
        if self.signed != right_nu.signed {
            panic!("Cannot compare signed with unsigned!");
        } else {
            let mut left_nu = self.clone();

            if self.signed {
                if left_nu.is_negative() && !right_nu.is_negative() {
                    return true;
                } else if !left_nu.is_negative() && right_nu.is_negative() {
                    return false;
                } else {
                    left_nu._minimum_width();
                    right_nu._minimum_width();

                    if (left_nu.size < right_nu.size) && !left_nu.is_negative() {
                        return true;
                    } else if (left_nu.size > right_nu.size) && left_nu.is_negative() {
                        return true;
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
    Otherwise it returns false. */
    pub fn gt(&self, mut right_nu: SvPrimaryLiteralIntegral) -> bool {
        if self.signed != right_nu.signed {
            panic!("Cannot compare signed with unsigned!");
        } else {
            let mut left_nu = self.clone();

            if self.signed {
                if left_nu.is_negative() && !right_nu.is_negative() {
                    return false;
                } else if !left_nu.is_negative() && right_nu.is_negative() {
                    return true;
                } else {
                    left_nu._minimum_width();
                    right_nu._minimum_width();

                    if (left_nu.size > right_nu.size) && !left_nu.is_negative() {
                        return true;
                    } else if (left_nu.size < right_nu.size) && left_nu.is_negative() {
                        return true;
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
    pub fn eq(&self, mut right_nu: SvPrimaryLiteralIntegral) -> bool {
        if self.signed != right_nu.signed {
            panic!("Cannot compare signed with unsigned!");
        } else {
            let mut left_nu = self.clone();

            if self.signed {
                if left_nu.is_negative() && !right_nu.is_negative() {
                    return false;
                } else if !left_nu.is_negative() && right_nu.is_negative() {
                    return false;
                } else {
                    left_nu._minimum_width();
                    right_nu._minimum_width();

                    if left_nu.size == right_nu.size {
                        return true;
                    }
                }
            } else {
                left_nu._minimum_width();
                right_nu._minimum_width();

                if left_nu.size == right_nu.size {
                    return true;
                }
            }

            false
        }
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

            self.size = size;
        } else {
            panic!("The original number of bits is smaller than the requested one!");
        }
    }

    pub fn add_usize(&self, right_nu: usize) -> SvPrimaryLiteralIntegral {
        let mut ret: SvPrimaryLiteralIntegral = self.clone();
        if !ret.signed {
            ret._unsigned_usize_add(right_nu);
            ret._minimum_width();
            ret = ret.to_4state();
        } else {
            let right_nu = SvPrimaryLiteralIntegral {
                data_01: vec![right_nu],
                data_xz: Some(vec![0]),
                size: usize::BITS as usize,
                signed: true,
            };

            ret = ret.add_primlit(right_nu.clone());
        }

        ret
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

        if !ret.contains_nonbinary() && !right_nu.contains_nonbinary() {
            if ret.signed == false || right_nu.signed == false {
                ret._unsigned_primlit_add(right_nu.clone());
                ret.signed = false;

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

                    ret._matched_sign_extension(&mut right_nu);
                    ret._unsigned_primlit_add(right_nu.clone());

                    new_size = (usize::BITS as usize - ret.data_01[0].leading_zeros() as usize)
                        + (ret.data_01.len() - 1) * usize::BITS as usize;
                    ret.size = new_size;

                    ret._minimum_width();
                } else {
                    ret._matched_sign_extension(&mut right_nu);
                    ret._unsigned_primlit_add(right_nu.clone());
                    ret._truncate(ret.size);

                    ret._minimum_width();
                }
            }

            if ret.is_4state() {
                ret = ret.to_4state();
            }

            ret
        } else {
            unimplemented!();
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

    pub fn mul(&self, mut right_nu: SvPrimaryLiteralIntegral) -> SvPrimaryLiteralIntegral {
        let mut left_nu: SvPrimaryLiteralIntegral = self.clone();
        let mut ret: SvPrimaryLiteralIntegral;

        if left_nu.is_4state() != right_nu.is_4state() {
            if !left_nu.is_4state() {
                left_nu.data_xz = Some(vec![0]);
            } else {
                right_nu.data_xz = Some(vec![0]);
            }
        }

        if !left_nu.contains_nonbinary() && !right_nu.contains_nonbinary() {
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
        } else {
            unimplemented!();
        }

        ret.data_xz = left_nu.data_xz.clone();
        if ret.is_4state() {
            if ret.data_01.len() != ret.data_xz.as_ref().unwrap().len() {
                for _x in 0..(ret.data_01.len() - ret.data_xz.as_ref().unwrap().len()) {
                    let mut new_vec = ret.data_xz.clone().unwrap();
                    new_vec.insert(0, 0);
                    ret.data_xz = Some(new_vec);
                }
            }
        }

        ret
    }
}

pub fn usize_to_primlit(value: usize) -> SvPrimaryLiteralIntegral {
    let mut ret = SvPrimaryLiteralIntegral {
        data_01: vec![value],
        data_xz: Some(vec![0]),
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
        write!(f, "Data_01: ")?;

        for x in 0..self.data_01.len() {
            writeln!(f, "{:b}", self.data_01[x])?;
        }

        write!(f, "Data_XZ: ")?;
        if !self.is_4state() {
            writeln!(f, "None")?;
        } else {
            for x in self.data_xz.as_ref().unwrap() {
                writeln!(f, "{:b}", x)?;
            }
        }

        write!(f, "")
    }
}

impl Ord for SvPrimaryLiteralIntegral {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other.clone()) {
            return Ordering::Equal;
        } else if self.lt(other.clone()) {
            return Ordering::Less;
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for SvPrimaryLiteralIntegral {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SvPrimaryLiteralIntegral {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other.clone())
    }
}

impl Eq for SvPrimaryLiteralIntegral {}
