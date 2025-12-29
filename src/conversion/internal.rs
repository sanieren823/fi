
use std::process::Output;

// TODO: consider that all input can be non-spruced-up values 
use crate::fi::FiBin;
use crate::fi::FiLong;


#[inline(always)]
pub fn low_bits(num: u128) -> u128 {
	num % 64
}

#[inline(always)]
pub fn high_bits(num: u128) -> u128 {
	num >> 64
}
impl FiLong {
	pub fn add(self, other: Self) -> Self {
		let mut carry: u128 = 0;
		let bigger: FiLong;
		let smaller: FiLong;
		if self.clone().abs() >= other.clone().abs() {
			bigger = self.clone();
			smaller = other.clone();
		} else {
			bigger = other.clone();
			smaller = self.clone();
		}
		let mut result: FiLong = FiLong::new(); // TODO: change
		for i in 0..bigger.len() {
			let res: u128;
			if smaller.len() > i {
				res = bigger[i] as u128 + carry;
			} else {
				res = bigger[i] as u128 + smaller[i] as u128 + carry;
			}
			carry = high_bits(res); // clone
			result.push(low_bits(res) as u64);

		}
		if carry  != 0{
			result.push(carry as u64);
		}
		result.sign = bigger.sign;
		result
	}

	pub fn sub(self, other: Self) -> Self {
		let mut borrow: u128 = 0;
		let bigger: FiLong;
		let smaller: FiLong;
		if self.clone().abs() >= other.clone().abs() {
			bigger = self.clone();
			smaller = other.clone();
		} else {
			bigger = other.clone();
			smaller = self.clone();
		}
		let mut result: FiLong = FiLong::new();
		for i in 0..bigger.len() {
			if smaller[i] + borrow as u64 > bigger[i] {
				result.push(u64::MAX - (smaller[i] + 1 - bigger[i]) - borrow as u64);
				borrow = 1;
			} else {
				result.push(bigger[i] - smaller[i] - borrow as u64);
				borrow = 0;
			}

		}
		result.sign = bigger.sign;
		result
	}

	pub fn mul(self, other: Self) -> Self {
		if self.is_zero() || other.is_zero() {
			return FiLong::new();
		}
		let mut carry: u128 = 0;
		let len = self.len() + other.len(); // consider spruce_up()
		let mut result: FiLong = FiLong{sign: self.sign ^ other.sign, value: vec![0; len]}; 
		for i in 0..self.len() {
			for j in 0..other.len() {
				let res: u128 = self[i] as u128 * other[j] as u128 + carry;
				carry = high_bits(res); // clone
				result[i + j] = low_bits(res) as u64;
			}

		}
		if carry != 0 {
			result[len - 1] = carry as u64;
		}
		result
	}
}








impl FiBin {
	pub fn to_long(&self) -> FiLong {
		let mut output = Vec::with_capacity((self.len() + 63) / 64); // faster than Vec::new()

		let mut qword: u64 = 0;
		let mut byte: u8 = 0;
		let mut cur_bit = 0; 
		let mut cur_byte = 0; 

		for &b in self.iter() {
			byte |= (b as u8) << cur_bit;
			cur_bit += 1;

			if cur_bit == 8 { // case: i % 8 == 0
				qword |= (byte as u64) << (cur_byte * 8);
				byte = 0;
				cur_bit = 0;
				cur_byte += 1;

				if cur_byte == 8 { // case: i % 64 == 0
					output.push(qword);
					qword = 0;
					cur_byte = 0;
				}
			}
		}

		
		if cur_bit != 0 || cur_byte != 0 {
			if cur_bit != 0 {
				qword |= (byte as u64) << (cur_byte * 8);
			}
			output.push(qword);
		}
		FiLong{sign: self.sign, value: output}
	}
} 


impl FiLong {
	pub fn to_bin(&self) -> FiBin {
		let mut output = Vec::with_capacity(self.len() * 64);
		for qword in self.iter() {
			let bytes: [u8; 8] = u64::to_le_bytes(*qword);
			for byte in bytes.iter() {
				for i in 0..8 {
					output.push((byte >> i & 1) != 0);
				}
			}
		}
		FiBin{sign: self.sign, value: output}.spruce_up()
	}
}

// implement long division? requires shr + shl implementation 