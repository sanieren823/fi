use crate::fi::fi;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign};

// TODO: write tests for all bitwise functions



impl BitAnd for fi {
    type Output = Self;


    fn bitand(self, rhs: Self) -> Self::Output {
        let mut new = fi::new();
        new.sign = self.sign & rhs.sign;
        let mut smallest: fi;
        let biggest: fi; 
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

impl BitAndAssign for fi {

    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.clone() & rhs
    }
}

impl BitOr for fi {
    type Output = Self;


    fn bitor(self, rhs: Self) -> Self::Output {
        let mut new = fi::new();
        new.sign = self.sign | rhs.sign;
        let mut smallest: fi;
        let biggest: fi; 
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

impl BitOrAssign for fi {

    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.clone() | rhs
    }
}

impl BitXor for fi {
    type Output = Self;


    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut new = fi::new();
        new.sign = self.sign ^ rhs.sign;
        let mut smallest: fi;
        let biggest: fi; 
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

impl BitXorAssign for fi {

    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.clone() ^ rhs
    }
}


impl Shl<usize> for fi {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut fi = self;
        for _ in 0..rhs {
            fi.insert(0, false);
        }
        fi
    }
}

impl ShlAssign<usize> for fi {
    fn shl_assign(&mut self, rhs: usize) {
        *self = self.clone() << rhs;
    }
}


impl Shr<usize> for fi {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut fi = self;
        for _ in 0..rhs {
            fi.remove(0);
        }
        fi
    }
}

impl ShrAssign<usize> for fi {
    fn shr_assign(&mut self, rhs: usize) {
        *self = self.clone() >> rhs;
    }
}