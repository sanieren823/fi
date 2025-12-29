use std::cmp::Ordering;
use crate::fi::{FiBin, FiLong};


impl PartialOrd for FiBin {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FiBin {
    fn cmp(&self, other: &Self) -> Ordering {

        match self.sign.cmp(&other.sign) {
            Ordering::Less => return Ordering::Greater,
            Ordering::Greater => return Ordering::Less,
            Ordering::Equal => (),
        }
        let v1 = self.spruce_up();    
        let v2 = other.spruce_up();
        let len_diff: isize = v1.len() as isize - v2.len() as isize;

        let mut b = false; // fix
        if len_diff == 0 {
            let len = v1.len();
            if len == 0 {
                return Ordering::Equal;
            }
            for i in 1..=len {
                if v1.value[len - i] != v2.value[len - i] {
                    b = v1.value[len - i];
                    break;
                } else if i == len {
                    return Ordering::Equal;
                }
            }
        } else {
            b = heaviside(&len_diff);
        }
        b ^= self.sign;
        if b {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialEq for FiBin {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.sign == other.sign
    }
}

impl Eq for FiBin {}

impl PartialOrd for FiLong {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FiLong {
    fn cmp(&self, other: &Self) -> Ordering {

        match self.sign.cmp(&other.sign) {
            Ordering::Less => return Ordering::Greater,
            Ordering::Greater => return Ordering::Less,
            Ordering::Equal => (),
        }
        let v1 = self.spruce_up();    
        let v2 = other.spruce_up();
        let len_diff: isize = v1.len() as isize - v2.len() as isize;

        let mut b = false; // fix
        if len_diff == 0 {
            let len = v1.len();
            if len == 0 {
                panic!("Make sure that your fi-object has a value.");
            }
            for i in 1..=len {
                if v1.value[len - i] != v2.value[len - i] {
                    b = v1.value[len - i] > v2.value[len - i];
                    break;
                } else if i == len {
                    return Ordering::Equal;
                }
            }
        } else {
            b = heaviside(&len_diff);
        }
        b ^= self.sign;
        if b {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialEq for FiLong {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.sign == other.sign
    }
}

impl Eq for FiLong {}

fn heaviside(num: &isize) -> bool {
    if *num < 0 {
        false
    } else {
        true
    }
}


