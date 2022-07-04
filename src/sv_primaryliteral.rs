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
        let left_size = self.num_bits;
        let right_size = right_nu.num_bits;

        // Ensure that their total nu of vector elements is the same in left and right
        if left_size > right_size {
            let diff: usize = (left_size - right_size) / usize::BITS as usize;

            for _x in 0..diff {
                right_nu.data01.insert(0, 0);
            }
        } else if left_size < right_size {
            let diff: usize = (right_size - left_size) / usize::BITS as usize;

            for _x in 0..diff {
                self.data01.insert(0, 0);
            }
        }
    }

    pub fn _signed_is_negative(&mut self) -> bool {
        let leading_plus_one: usize = usize::BITS as usize
            - (self.num_bits - 1 - (self.data01.len() - 1) * usize::BITS as usize);

        if !self.data01[0].leading_zeros() as usize == leading_plus_one {
            true
        } else {
            false
        }
    }

    pub fn _signed_matched_sign_extension(&mut self, right_nu: &mut SvPrimaryLiteral) {
        let _left_neg: bool = self._signed_is_negative();
        let _right_neg: bool = right_nu._signed_is_negative();

        self._prim_lit_vec_elmnt_match(right_nu);

        unimplemented!();
        // TODO
    }

    pub fn _signed_sign_inversion(&mut self) {
        let from_negative: bool = self._signed_is_negative();

        for x in (0..self.data01.len()).rev() {
            self.data01[x] = !self.data01[x];
        }

        if from_negative {
            let pre_inv: SvPrimaryLiteral = self.clone();
            self._unsigned_usize_add(1);

            self.num_bits = (usize::BITS as usize - self.data01[0].leading_zeros() as usize + 1)
                + (self.data01.len() - 1) * usize::BITS as usize;

            if self.data01 == pre_inv.data01 && self.num_bits == pre_inv.num_bits {
                if self.data01[0].leading_zeros() == 0 {
                    self.data01.insert(0, 0);
                    self.num_bits = self.num_bits + 1;
                } else {
                    self.num_bits = self.num_bits + 1;
                }
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
                    self.data01[x] - 2usize.pow(usize::BITS - pre_leading - 1);
                let post_leading = minimized_value.leading_zeros();

                if (post_leading != pre_leading - 1)
                    || (post_leading == usize::BITS && (x == self.data01.len() - 1))
                {
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

    pub fn usize_add(&mut self, right_nu: usize) {
        if !self.signed {
            let new_num_bits: usize;

            self._unsigned_usize_add(right_nu);

            new_num_bits = (usize::BITS as usize - self.data01[0].leading_zeros() as usize)
                + (self.data01.len() - 1) * usize::BITS as usize;
            self.num_bits = new_num_bits;
        } else {
            let left_neg: bool = self._signed_is_negative();
            let mut right_neg: bool = false;
            let r_leading: usize = right_nu.leading_zeros() as usize;
            if r_leading == 0 {
                right_neg = true;
            }

            if !left_neg && !right_neg {
                let new_num_bits: usize;

                self._unsigned_usize_add(right_nu);

                if self.data01[0].leading_zeros() == 0 {
                    self.data01.insert(0, 0);
                }

                new_num_bits = (usize::BITS as usize - self.data01[0].leading_zeros() as usize + 1)
                    + (self.data01.len() - 1) * usize::BITS as usize;

                self.num_bits = new_num_bits;
            } else if left_neg && right_neg {
                // Make both operands +ve
                let mut right_nu = SvPrimaryLiteral {
                    data01: vec![right_nu],
                    num_bits: usize::BITS as usize,
                    signed: true,
                };

                self._signed_sign_inversion();
                right_nu._signed_sign_inversion();
                self._unsigned_prim_lit_add(right_nu.clone());
                self._signed_sign_inversion();
            }
        }

        println!("The new data01 is:");
        for x in 0..self.data01.len() {
            println!("{:b}", self.data01[x]);
        }
        println!("The new num of bits is: {} \n", self.num_bits);
    }

    pub fn prim_lit_add(&mut self, right_nu: SvPrimaryLiteral) {
        if self.signed == false || right_nu.signed == false {
            self._unsigned_prim_lit_add(right_nu.clone());

            let new_num_bits: usize = (usize::BITS as usize
                - self.data01[0].leading_zeros() as usize)
                + (self.data01.len() - 1) * usize::BITS as usize;
            self.num_bits = new_num_bits;
        }

        println!("The new data01 is:");
        for x in 0..self.data01.len() {
            println!("{:b}", self.data01[x]);
        }
        println!("The new num of bits is: {} \n", self.num_bits);
    }
}
