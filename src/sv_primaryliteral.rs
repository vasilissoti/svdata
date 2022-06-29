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

            new_num_bits =
                (64 - self.data01[0].leading_zeros() as usize) + (self.data01.len() - 1) * 64;
            self.num_bits = new_num_bits;

            println!("The new data01 is:");
            for x in 0..self.data01.len() {
                println!("{:b}", self.data01[x]);
            }
            println!("The new num of bits is: {} \n", self.num_bits);
        }
    }
}
