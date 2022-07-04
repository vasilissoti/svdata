#[derive(Debug, Clone)]
pub struct SvPrimaryLiteral {
    pub data01: Vec<usize>,
    pub num_bits: usize,
    pub signed: bool,
}

impl SvPrimaryLiteral {
    // The following functions should be replaced by the build in methods once they become stable
    pub fn usize_add(&mut self, right_nu: usize) {
        if !self.signed {
            let new_num_bits: usize;
            let last_index = self.data01.len() - 1;
            let left_nu: usize = self.data01[last_index];
            self.data01[last_index] = left_nu.wrapping_add(right_nu);

            if (self.data01[last_index] < left_nu) || (self.data01[last_index] < right_nu) {
                if self.data01.len() == 1 {
                    self.data01.insert(0, 1);
                } else {
                    let mut carry_flag: bool = true;

                    for x in (0..self.data01.len() - 1).rev() {
                        let left_nu: usize = self.data01[x];
                        self.data01[x] = left_nu.wrapping_add(1);

                        if self.data01[x] >= left_nu {
                            carry_flag = false;
                            break;
                        }
                    }

                    if carry_flag {
                        self.data01.insert(0, 1);
                    }
                }
            }

            new_num_bits = (usize::BITS as usize - self.data01[0].leading_zeros() as usize)
                + (self.data01.len() - 1) * usize::BITS as usize;
            self.num_bits = new_num_bits;
        } else {
            let mut left_neg: bool = false;
            let mut right_neg: bool = false;

            let l_leading_plus_one: usize = usize::BITS as usize
                - (self.num_bits - 1 - (self.data01.len() - 1) * usize::BITS as usize);

            let r_leading: usize = right_nu.leading_zeros() as usize;

            if !self.data01[0].leading_zeros() as usize == l_leading_plus_one {
                left_neg = true;
            }

            if r_leading == 0 {
                right_neg = true;
            }

            if !left_neg && !right_neg {
                let new_num_bits: usize;
                let last_index = self.data01.len() - 1;
                let left_nu: usize = self.data01[last_index];
                self.data01[last_index] = left_nu.wrapping_add(right_nu);

                if (self.data01[last_index] < left_nu) || (self.data01[last_index] < right_nu) {
                    if self.data01.len() == 1 {
                        self.data01.insert(0, 1);
                    } else {
                        let mut carry_flag: bool = true;

                        for x in (0..self.data01.len() - 1).rev() {
                            let left_nu: usize = self.data01[x];
                            self.data01[x] = left_nu.wrapping_add(1);

                            if self.data01[x] >= left_nu {
                                carry_flag = false;
                                break;
                            }
                        }

                        if carry_flag {
                            self.data01.insert(0, 1);
                        }
                    }
                }

                if self.data01[0].leading_zeros() == 0 {
                    self.data01.insert(0, 0);
                }

                new_num_bits = (usize::BITS as usize - self.data01[0].leading_zeros() as usize + 1)
                    + (self.data01.len() - 1) * usize::BITS as usize;

                self.num_bits = new_num_bits;
            }
        }

        println!("The new data01 is:");
        for x in 0..self.data01.len() {
            println!("{:b}", self.data01[x]);
        }
        println!("The new num of bits is: {} \n", self.num_bits);
    }

    pub fn prim_lit_add(&mut self, mut right_nu: SvPrimaryLiteral) {
        if self.signed == false || right_nu.signed == false {
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

            let new_num_bits: usize = (usize::BITS as usize
                - self.data01[0].leading_zeros() as usize)
                + (self.data01.len() - 1) * usize::BITS as usize;
            self.num_bits = new_num_bits;

            println!("The new data01 is:");
            for x in 0..self.data01.len() {
                println!("{:b}", self.data01[x]);
            }
            println!("The new num of bits is: {} \n", self.num_bits);
        }
    }
}
