use std::fmt;

#[derive(Debug, Clone)]
pub struct SvPrimaryLiteral {
    pub data01: Vec<usize>,
    pub num_bits: usize,
    pub signed: bool,
}

// The following functions should be replaced by the build in methods once they become stable
impl SvPrimaryLiteral {
    /* Unsigned addition between a primary literal and usize (since these can be used for "signed" and "unsigned" values, the final number of bits is not derived within the function but instead must be explicitly implemented according the context that the function is used). */
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

    /* Unsigned addition between two primary literals. Both data01 vector dimensions (i.e nu of elements) are matched. As far as the final number of bits is concerned, the same logic was applied as the one mentioned in the previous fn). */
    pub fn _unsigned_primlit_add(&mut self, mut right_nu: SvPrimaryLiteral) {
        self._primlit_vec_elmnt_match(&mut right_nu);

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

    /* Accepts two primary literals and ensures that both data01 vector dimensions (i.e nu of elements) are matched. */
    pub fn _primlit_vec_elmnt_match(&mut self, right_nu: &mut SvPrimaryLiteral) {
        let left_size = self.data01.len();
        let right_size = right_nu.data01.len();

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

    /* Receives a signed primary literal as an argument and deduces whether the stored value is -ve or +ve based on the num_bits value set. */
    pub fn is_negative(&mut self) -> bool {
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

    /* Receives a primary literal as an argument and deduces whether the stored value is zero. */
    pub fn is_zero(&mut self) -> bool {
        for x in 0..self.data01.len() {
            if self.data01[x].leading_zeros() != usize::BITS {
                return false;
            }
        }

        true
    }

    /* Accepts two signed primary literals and ensures that both are properly sign extended and matched to their data01 dimensions. The correct final number of bits is set to both arguments. */
    pub fn _matched_sign_extension(&mut self, right_nu: &mut SvPrimaryLiteral) {
        if self.signed != true || right_nu.signed != true {
            panic!("Expected signed SvPrimaryLiterals but found unsigned!");
        }

        let left_neg: bool = self.is_negative();
        let right_neg: bool = right_nu.is_negative();

        self._primlit_vec_elmnt_match(right_nu);

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

    /* Receives a signed primary literal and sign extends the value in the existing number of vector elements. The correct final number of bits is set to the argument. */
    pub fn _sign_extension(&mut self) {
        if self.signed != true {
            panic!("Expected signed SvPrimaryLiteral but found unsigned!");
        }

        let left_neg: bool = self.is_negative();

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

    /* Receives a signed primary literal and returns its opposite signed primary literal (i.e +ve -> -ve and vice versa). The correct final number of bits is set to the argument. */
    pub fn neg(&self) -> SvPrimaryLiteral {
        let mut ret: SvPrimaryLiteral = self.clone();
        if ret.is_zero() {
            return ret;
        } else if ret.signed != true {
            panic!("Expected signed SvPrimaryLiteral but found unsigned!");
        }

        let from_negative: bool = ret.is_negative();
        ret._sign_extension();

        ret = ret.inv();

        ret._unsigned_usize_add(1);

        if from_negative {
            ret.num_bits = (usize::BITS as usize - ret.data01[0].leading_zeros() as usize + 1)
                + (ret.data01.len() - 1) * usize::BITS as usize;

            if ret.data01[0].leading_zeros() == 0 {
                ret.data01.insert(0, 0);
            }
        } else {
            ret.num_bits = (usize::BITS as usize - ret.data01[0].leading_zeros() as usize)
                + (ret.data01.len() - 1) * usize::BITS as usize;
            ret._minimum_width();
        }

        ret
    }

    /* Receives a signed primary literal and returns a primary literal with its inverted value. The final number of bits remains the same as the original one.*/
    pub fn inv(&self) -> SvPrimaryLiteral {
        let mut ret: SvPrimaryLiteral = self.clone();

        for x in (0..ret.data01.len()).rev() {
            let mut lsl: usize = ret.data01[x];
            for y in 0..usize::BITS {
                if lsl.leading_zeros() == 0 {
                    ret.data01[x] = ret.data01[x] - 2usize.pow(usize::BITS - y - 1);
                } else {
                    ret.data01[x] = ret.data01[x] + 2usize.pow(usize::BITS - y - 1);
                }

                lsl = lsl << 1;
            }
        }

        ret
    }

    /* Receives a signed or unsigned primary literal and deduces an equivalent representation with the minimum number of bits required. The correct final number of bits is set to the argument. */
    pub fn _minimum_width(&mut self) {
        if !self.signed {
            if self.is_zero() {
                for _x in 0..self.data01.len() {
                    self.data01.remove(0);
                }
                self.data01.push(0);
                self.num_bits = 1;
            } else {
                for _x in 0..self.data01.len() {
                    if self.data01[0] == 0 {
                        self.data01.remove(0);
                    }
                }

                self.num_bits = (usize::BITS as usize - self.data01[0].leading_zeros() as usize)
                    + (self.data01.len() - 1) * usize::BITS as usize;
            }
        } else {
            let mut min_num_found: bool = false;
            let mut vec_elements_to_rm: usize = 0;

            if self.is_negative() {
                for x in 0..self.data01.len() {
                    while !min_num_found {
                        let pre_leading = self.data01[x].leading_zeros();

                        let minimized_value: usize =
                            self.data01[x] - 2usize.pow(usize::BITS - pre_leading - 1); //TODO
                        let post_leading = minimized_value.leading_zeros();

                        if post_leading == usize::BITS {
                            if x == (self.data01.len() - 1)
                                || self.data01[x + 1].leading_zeros() != 0
                            {
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
            } else if self.is_zero() {
                for _x in 0..self.data01.len() {
                    self.data01.remove(0);
                }
                self.data01.push(0);
                self.num_bits = 1;
            } else {
                for _x in 0..self.data01.len() {
                    if self.data01[0] == 0 {
                        self.data01.remove(0);
                    }
                }

                if self.data01[0].leading_zeros() == 0 {
                    self.data01.insert(0, 0);
                }

                self.num_bits = (usize::BITS as usize - self.data01[0].leading_zeros() as usize
                    + 1)
                    + (self.data01.len() - 1) * usize::BITS as usize;
            }
        }
    }

    /* Receives the number of bits in which a primary literal should be truncated. The correct final number of bits is set but the signedness doesn't change. */
    pub fn _truncate(&mut self, num_bits: usize) {
        if num_bits == 0 {
            panic!("Cannot truncate the value to zero bits!");
        } else if self.num_bits >= num_bits {
            let elmnts_to_be_rm: usize;

            if (num_bits % usize::BITS as usize) == 0 {
                elmnts_to_be_rm = self.data01.len() - num_bits / usize::BITS as usize;
            } else {
                elmnts_to_be_rm = self.data01.len() - (num_bits / usize::BITS as usize) - 1;
            }

            let bits_to_be_rm: usize = (self.num_bits - num_bits) % usize::BITS as usize;

            let locator: usize;
            if self.num_bits % usize::BITS as usize == 0 {
                locator = usize::BITS as usize;
            } else {
                locator = self.num_bits % usize::BITS as usize;
            }

            for _x in 0..elmnts_to_be_rm {
                self.data01.remove(0);
            }

            if bits_to_be_rm != 0 {
                for x in ((locator - bits_to_be_rm)..(locator + 1)).rev() {
                    if self.data01[0].leading_zeros() == (usize::BITS - x as u32) {
                        self.data01[0] = self.data01[0] - 2usize.pow(x as u32 - 1);
                    }
                }
            }

            self.num_bits = num_bits;
        } else {
            panic!("The original number of bits is smaller than the requested one!");
        }
    }

    pub fn add_usize(&self, right_nu: usize) -> SvPrimaryLiteral {
        let mut ret: SvPrimaryLiteral = self.clone();
        if !ret.signed {
            ret._unsigned_usize_add(right_nu);
            ret._minimum_width();
            ret
        } else {
            let right_nu = SvPrimaryLiteral {
                data01: vec![right_nu],
                num_bits: usize::BITS as usize,
                signed: true,
            };

            ret.add_primlit(right_nu.clone())
        }
    }

    pub fn add_primlit(&self, mut right_nu: SvPrimaryLiteral) -> SvPrimaryLiteral {
        let mut ret: SvPrimaryLiteral = self.clone();
        if ret.signed == false || right_nu.signed == false {
            ret._unsigned_primlit_add(right_nu.clone());
            ret.signed = false;

            ret._minimum_width();
        } else {
            let left_neg: bool = ret.is_negative();
            let right_neg: bool = right_nu.is_negative();

            if !left_neg && !right_neg {
                let new_num_bits: usize;

                ret._unsigned_primlit_add(right_nu.clone());

                if ret.data01[0].leading_zeros() == 0 {
                    ret.data01.insert(0, 0);
                }

                new_num_bits = (usize::BITS as usize - ret.data01[0].leading_zeros() as usize + 1)
                    + (ret.data01.len() - 1) * usize::BITS as usize;

                ret.num_bits = new_num_bits;
            } else if left_neg && right_neg {
                let new_num_bits: usize;

                ret._matched_sign_extension(&mut right_nu);
                ret._unsigned_primlit_add(right_nu.clone());

                new_num_bits = (usize::BITS as usize - ret.data01[0].leading_zeros() as usize)
                    + (ret.data01.len() - 1) * usize::BITS as usize;
                ret.num_bits = new_num_bits;

                ret._minimum_width();
            } else {
                let new_num_bits: usize;

                ret._matched_sign_extension(&mut right_nu);
                ret._unsigned_primlit_add(right_nu.clone());
                ret._truncate(ret.num_bits);

                if ret.is_negative() {
                    ret._minimum_width();
                } else if ret.is_zero() {
                    ret._truncate(usize::BITS as usize);
                    ret.num_bits = 1;
                } else {
                    new_num_bits = (usize::BITS as usize - ret.data01[0].leading_zeros() as usize
                        + 1)
                        + (ret.data01.len() - 1) * usize::BITS as usize;
                    ret.num_bits = new_num_bits;
                }
            }
        }
        ret
    }
}

pub fn usize_to_primlit(value: usize) -> SvPrimaryLiteral {
    let mut ret = SvPrimaryLiteral {
        data01: vec![value],
        num_bits: usize::BITS as usize,
        signed: true,
    };

    ret._minimum_width();

    ret
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
