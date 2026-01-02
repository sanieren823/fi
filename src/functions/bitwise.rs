use crate::fi::{FiBin, FiLong};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign};

// TODO: write tests for all bitwise functions



impl BitAnd for FiBin {
    type Output = Self;


    fn bitand(self, rhs: Self) -> Self::Output {
        let mut new = FiBin::new();
        new.sign = self.sign & rhs.sign;
        let mut smallest: FiBin;
        let biggest: FiBin; 
        if self.len() >= rhs.len() {
            smallest = rhs;
            biggest = self;
        } else {
            smallest = self;
            biggest = rhs;
        }
        for _ in smallest.len()..biggest.len() {
            smallest.push(false);
        }
        for i in 0..biggest.len() {
            new.push(smallest[i] & biggest[i]);
        }
        new
    }
}

impl BitAndAssign for FiBin {

    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.clone() & rhs
    }
}

impl BitOr for FiBin {
    type Output = Self;


    fn bitor(self, rhs: Self) -> Self::Output {
        let mut new = FiBin::new();
        new.sign = self.sign | rhs.sign;
        let mut smallest: FiBin;
        let biggest: FiBin; 
        if self.len() >= rhs.len() {
            smallest = rhs;
            biggest = self;
        } else {
            smallest = self;
            biggest = rhs;
        }
        for _ in smallest.len()..biggest.len() {
            smallest.push(false);
        }
        for i in 0..biggest.len() {
            new.push(smallest[i] | biggest[i]);
        }
        new
    }
}

impl BitOrAssign for FiBin {

    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.clone() | rhs
    }
}

impl BitXor for FiBin {
    type Output = Self;


    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut new = FiBin::new();
        new.sign = self.sign ^ rhs.sign;
        let mut smallest: FiBin;
        let biggest: FiBin; 
        if self.len() >= rhs.len() {
            smallest = rhs;
            biggest = self;
        } else {
            smallest = self;
            biggest = rhs;
        }
        for _ in smallest.len()..biggest.len() {
            smallest.push(false);
        }
        for i in 0..biggest.len() {
            new.push(smallest[i] ^ biggest[i]);
        }
        new
    }
}

impl BitXorAssign for FiBin {

    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.clone() ^ rhs
    }
}


impl Shl<usize> for FiBin {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut fi = self;
        for _ in 0..rhs {
            fi.insert(0, false);
        }
        fi
    }
}

impl ShlAssign<usize> for FiBin {
    fn shl_assign(&mut self, rhs: usize) {
        *self = self.clone() << rhs;
    }
}


impl Shr<usize> for FiBin {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut fi = self;
        for i in 0..rhs {
            if i >= fi.len() {
                break;
            }
            fi.remove(0);
        }
        fi
    }
}

impl ShrAssign<usize> for FiBin {
    fn shr_assign(&mut self, rhs: usize) {
        *self = self.clone() >> rhs;
    }
}



















impl BitAnd for FiLong {
    type Output = Self;


    fn bitand(self, rhs: Self) -> Self::Output {
        let mut new = FiLong::new();
        new.sign = self.sign & rhs.sign;
        let mut smallest: FiLong;
        let biggest: FiLong; 
        if self.len() >= rhs.len() {
            smallest = rhs;
            biggest = self;
        } else {
            smallest = self;
            biggest = rhs;
        }
        for _ in smallest.len()..biggest.len() {
            smallest.push(0);
        }
        for i in 0..biggest.len() {
            new.push(smallest[i] & biggest[i]);
        }
        new
    }
}

impl BitAndAssign for FiLong {

    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.clone() & rhs
    }
}

impl BitOr for FiLong {
    type Output = Self;


    fn bitor(self, rhs: Self) -> Self::Output {
        let mut new = FiLong::new();
        new.sign = self.sign & rhs.sign;
        let mut smallest: FiLong;
        let biggest: FiLong; 
        if self.len() >= rhs.len() {
            smallest = rhs;
            biggest = self;
        } else {
            smallest = self;
            biggest = rhs;
        }
        for _ in smallest.len()..biggest.len() {
            smallest.push(0);
        }
        for i in 0..biggest.len() {
            new.push(smallest[i] | biggest[i]);
        }
        new
    }
}

impl BitOrAssign for FiLong {

    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.clone() | rhs
    }
}

impl BitXor for FiLong {
    type Output = Self;


    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut new = FiLong::new();
        new.sign = self.sign & rhs.sign;
        let mut smallest: FiLong;
        let biggest: FiLong; 
        if self.len() >= rhs.len() {
            smallest = rhs;
            biggest = self;
        } else {
            smallest = self;
            biggest = rhs;
        }
        for _ in smallest.len()..biggest.len() {
            smallest.push(0);
        }
        for i in 0..biggest.len() {
            new.push(smallest[i] ^ biggest[i]);
        }
        new
    }
}

impl BitXorAssign for FiLong {

    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.clone() ^ rhs
    }
}


impl Shl<usize> for FiLong {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        let full = rhs / 64;
        let exact = rhs % 64;
        let mut output: Vec<u64> = Vec::with_capacity(self.len() + full + 1);
        for _ in 0..full {
            output.push(0);
        }
        output.push(0);
        for i in 0..self.len() {
            let this = self[i] << exact;
            if exact == 0 {
                output[i + full] |= this;
                break;
            }
            
            let next = self[i] >> (64 - exact);
            output[i + full] |= this;
            if next > 0  || i + 1 < self.len() {
                output.push(next);
            }
            
            
        }
        FiLong{sign: self.sign, value: output}
    }
}

impl ShlAssign<usize> for FiLong {
    fn shl_assign(&mut self, rhs: usize) {
        *self = self.clone() << rhs;
    }
}


impl Shr<usize> for FiLong {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        if rhs >= self.len() * 64 {
            FiLong::new()
        } else {
            let mut fi = self;
            let full = rhs / 64;
            let exact = rhs % 64;
            for _ in 0..full {
                fi.remove(0);
            }
            let mut output: Vec<u64> = Vec::with_capacity(fi.value.capacity());
            for i in 0..fi.len() {
                if exact == 0 {
                    break;
                }
                let this = fi[i] >> exact;
                let prev = fi[i] << (64 - exact);
                output.push(this);
                if i > 0 {
                    output[i - 1] |= prev;
                }
            }
            FiLong{sign: fi.sign, value: output}
        }
    }
}

impl ShrAssign<usize> for FiLong {
    fn shr_assign(&mut self, rhs: usize) {
        *self = self.clone() >> rhs;
    }
}