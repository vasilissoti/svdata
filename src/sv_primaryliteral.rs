use std::fmt;

#[derive(Debug, Clone)]
pub struct SvPrimaryLiteral {
    pub data01: Vec<usize>,
    pub num_bits: usize,
    pub signed: bool,
}

impl SvPrimaryLiteral {
    // The following functions should be replaced by the build in methods once they become stable
    pub fn _unsigned_usize_add(&mut self, right_nu: usize) {
        let last_index = self.data01.len() - 1;
        let left_nu: usize = self.data01[last_index];
        self.data01[last_index] = left_nu.wrapping_add(right_nu.clone());

        if (self.data01[last_index] < left_nu) || (self.data01[last_index] < right_nu.clone()) {
            if self.data01.len() == 1 {
                self.data01.insert(0, 1);
            } else {
                let mut carry_flag: bool = true;

                for x in (0..self.data01.len() - 1).rev() {
                    let left_nu: usize = self.data01[x];
                    self.data01[x] = left_nu.wrapping_add(1);

                    if self.data01[x] > left_nu {
                        carry_flag = false;
                        break;
                    }
                }

                if carry_flag {
                    self.data01.insert(0, 1);
                }
            }
        }
    }

    pub fn _unsigned_prim_lit_add(&mut self, mut right_nu: SvPrimaryLiteral) {
        self._prim_lit_vec_elmnt_match(&mut right_nu);

        let mut carry_flag: bool = false;

        for x in (0..self.data01.len()).rev() {
            let left_nu: usize = self.data01[x];
            self.data01[x] = left_nu.wrapping_add(right_nu.data01[x]);

            if carry_flag {
                self.data01[x] = self.data01[x].wrapping_add(1);
            }

            if self.data01[x] >= left_nu && self.data01[x] >= right_nu.data01[x] {
                carry_flag = false;
            } else {
                carry_flag = true;
            }
        }

        if carry_flag {
            self.data01.insert(0, 1);
        }
    }

    pub fn _prim_lit_vec_elmnt_match(&mut self, right_nu: &mut SvPrimaryLiteral) {
        let left_size = self.data01.len();
        let right_size = right_nu.data01.len();

        // Ensure that their total nu of vector elements is the same in left and right
        if left_size > right_size {
            let diff: usize = left_size - right_size;

            for _x in 0..diff {
                right_nu.data01.insert(0, 0);
            }
        } else if left_size < right_size {
            let diff: usize = right_size - left_size;

            for _x in 0..diff {
                self.data01.insert(0, 0);
            }
        }
    }

    pub fn _signed_is_negative(&mut self) -> bool {
        if self.signed != true {
            panic!("Expected signed SvPrimaryLiteral but found unsigned!");
        }

        let leading_zeros: usize =
            usize::BITS as usize - (self.num_bits - (self.data01.len() - 1) * usize::BITS as usize);

        if self.data01[0].leading_zeros() as usize == leading_zeros {
            true
        } else {
            false
        }
    }

    pub fn _is_zero(&mut self) -> bool {
        if self.data01.len() == 1 && self.data01[0].leading_zeros() == usize::BITS {
            true
        } else {
            false
        }
    }

    pub fn _signed_matched_sign_extension(&mut self, right_nu: &mut SvPrimaryLiteral) {
        if self.signed != true || right_nu.signed != true {
            panic!("Expected signed SvPrimaryLiterals but found unsigned!");
        }

        let left_neg: bool = self._signed_is_negative();
        let right_neg: bool = right_nu._signed_is_negative();

        self._prim_lit_vec_elmnt_match(right_nu);

        if left_neg {
            let mut last_element: bool = false;

            for x in 0..self.data01.len() {
                let left_leading = self.data01[x].leading_zeros();

                if left_leading != usize::BITS {
                    last_element = true;
                }

                for y in 0..left_leading {
                    self.data01[x] = self.data01[x] + 2usize.pow(usize::BITS - y - 1);
                }

                if last_element {
                    break;
                }
            }
        }

        if right_neg {
            let mut last_element: bool = false;

            for x in 0..right_nu.data01.len() {
                let left_leading = right_nu.data01[x].leading_zeros();

                if left_leading != usize::BITS {
                    last_element = true;
                }

                for y in 0..left_leading {
                    right_nu.data01[x] = right_nu.data01[x] + 2usize.pow(usize::BITS - y - 1);
                }

                if last_element {
                    break;
                }
            }
        }

        self.num_bits = self.data01.len() * usize::BITS as usize;
        right_nu.num_bits = right_nu.data01.len() * usize::BITS as usize;
    }

    pub fn _signed_sign_extension(&mut self) {
        let left_neg: bool = self._signed_is_negative();

        if left_neg {
            let mut last_element: bool = false;

            for x in 0..self.data01.len() {
                let left_leading = self.data01[x].leading_zeros();

                if left_leading != usize::BITS {
                    last_element = true;
                }

                for y in 0..left_leading {
                    self.data01[x] = self.data01[x] + 2usize.pow(usize::BITS - y - 1);
                }

                if last_element {
                    break;
                }
            }
        }

        self.num_bits = self.data01.len() * usize::BITS as usize;
    }

    pub fn _signed_sign_inversion(&mut self) {
        if self._is_zero() {
            return;
        } else if self.signed != true {
            panic!("Expected signed SvPrimaryLiteral but found unsigned!");
        }

        let from_negative: bool = self._signed_is_negative();
        self._signed_sign_extension();

        for x in (0..self.data01.len()).rev() {
            self.data01[x] = !self.data01[x];
        }

        if from_negative {
            self._unsigned_usize_add(1);

            self.num_bits = (usize::BITS as usize - self.data01[0].leading_zeros() as usize + 1)
                + (self.data01.len() - 1) * usize::BITS as usize;

            if self.data01[0].leading_zeros() == 0 {
                self.data01.insert(0, 0);
            }
        } else {
            self._unsigned_usize_add(1);
            self.num_bits = (usize::BITS as usize - self.data01[0].leading_zeros() as usize)
                + (self.data01.len() - 1) * usize::BITS as usize;
            self._neg_value_num_bit_minimizer();
        }
    }

    pub fn _neg_value_num_bit_minimizer(&mut self) {
        let mut min_num_found: bool = false;
        let mut vec_elements_to_rm: usize = 0;

        for x in 0..self.data01.len() {
            while !min_num_found {
                let pre_leading = self.data01[x].leading_zeros();

                let minimized_value: usize =
                    self.data01[x] - 2usize.pow(usize::BITS - pre_leading - 1); //TODO
                let post_leading = minimized_value.leading_zeros();

                if post_leading == usize::BITS {
                    if x == (self.data01.len() - 1) || self.data01[x + 1].leading_zeros() != 0 {
                        min_num_found = true;
                        break;
                    }
                }

                if post_leading != (pre_leading + 1) {
                    min_num_found = true;
                    break;
                } else {
                    self.data01[x] = minimized_value;
                    self.num_bits = self.num_bits - 1;

                    if post_leading == usize::BITS {
                        vec_elements_to_rm = vec_elements_to_rm + 1;
                        break;
                    }
                }
            }
        }

        for _x in 0..vec_elements_to_rm {
            self.data01.remove(0);
        }
    }

    pub fn _truncate_size(&mut self, num_bits: usize) {
        if num_bits == 0 {
            panic!("Cannot truncate the value to zero bits!");
        } else if self.num_bits >= num_bits {
            let elmnts_to_be_rm: usize = self.data01.len() - num_bits / usize::BITS as usize;
            println!("{}", elmnts_to_be_rm);
            let bits_to_be_rm: usize = (self.num_bits - num_bits) % usize::BITS as usize;
            println!("{}", bits_to_be_rm);

            let locator: usize;
            if elmnts_to_be_rm == 0 {
                locator = usize::BITS as usize - self.num_bits;
            } else {
                locator = usize::BITS as usize;
            }
            println!("{}", locator);

            for _x in 0..elmnts_to_be_rm {
                self.data01.remove(0);
            }

            for x in locator..(locator - bits_to_be_rm) {
                if (self.data01[0].leading_zeros() as usize) == (usize::BITS as usize - x) {
                    self.data01[0] = self.data01[0] - 2usize.pow(x as u32 - 1);
                }
            }

            self.num_bits = num_bits;
        } else {
            panic!("The original number of bits is smaller than the requested one!");
        }
    }

    pub fn usize_add(&mut self, right_nu: usize) {
        if !self.signed {
            let new_num_bits: usize;

            self._unsigned_usize_add(right_nu);
            if !self._is_zero() {
                new_num_bits = (usize::BITS as usize - self.data01[0].leading_zeros() as usize)
                    + (self.data01.len() - 1) * usize::BITS as usize;
                self.num_bits = new_num_bits;
            } else {
                self.num_bits = 1;
            }
        } else {
            let right_nu = SvPrimaryLiteral {
                data01: vec![right_nu],
                num_bits: usize::BITS as usize,
                signed: true,
            };

            self.prim_lit_add(right_nu.clone());
        }
    }

    pub fn prim_lit_add(&mut self, mut right_nu: SvPrimaryLiteral) {
        if self.signed == false || right_nu.signed == false {
            self._unsigned_prim_lit_add(right_nu.clone());
            self.signed = false;

            let new_num_bits: usize;
            if !self._is_zero() {
                new_num_bits = (usize::BITS as usize - self.data01[0].leading_zeros() as usize)
                    + (self.data01.len() - 1) * usize::BITS as usize;
                self.num_bits = new_num_bits;
            } else {
                self.num_bits = 1;
            }
        } else {
            let left_neg: bool = self._signed_is_negative();
            let right_neg: bool = right_nu._signed_is_negative();

            if !left_neg && !right_neg {
                let new_num_bits: usize;

                self._unsigned_prim_lit_add(right_nu.clone());

                if self.data01[0].leading_zeros() == 0 {
                    self.data01.insert(0, 0);
                }

                new_num_bits = (usize::BITS as usize - self.data01[0].leading_zeros() as usize + 1)
                    + (self.data01.len() - 1) * usize::BITS as usize;

                self.num_bits = new_num_bits;
            } else if left_neg && right_neg {
                let new_num_bits: usize;

                self._signed_matched_sign_extension(&mut right_nu);
                self._unsigned_prim_lit_add(right_nu.clone());

                new_num_bits = (usize::BITS as usize - self.data01[0].leading_zeros() as usize)
                    + (self.data01.len() - 1) * usize::BITS as usize;
                self.num_bits = new_num_bits;

                self._neg_value_num_bit_minimizer();
            } else {
                let new_num_bits: usize;

                self._signed_matched_sign_extension(&mut right_nu);
                self._unsigned_prim_lit_add(right_nu.clone());
                self._truncate_size(self.num_bits);

                if self._signed_is_negative() {
                    self._neg_value_num_bit_minimizer();
                } else if self._is_zero() {
                    self._truncate_size(usize::BITS as usize);
                    self.num_bits = 1;
                } else {
                    new_num_bits = (usize::BITS as usize - self.data01[0].leading_zeros() as usize
                        + 1)
                        + (self.data01.len() - 1) * usize::BITS as usize;
                    self.num_bits = new_num_bits;
                }
            }
        }
    }
}

impl fmt::Display for SvPrimaryLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "Data01: ")?;

        for x in 0..self.data01.len() {
            write!(f, "{:b} ", self.data01[x])?;
        }

        write!(f, "\n")?;

        writeln!(f, "NumBits: {}", self.num_bits)?;
        write!(f, "Signed: {}", self.signed)
    }
}
