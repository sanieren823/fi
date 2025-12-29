mod fi;
mod errors;
mod conversion;
use crate::fi::Parsing;
// use crate::conversion::int::ParseInt;
mod functions;
use crate::fi::FiBin as fixed_int;
use std::time::Instant;

fn main() {
    // let fixed = fi::fi{sign: false, value: vec![false, true, false, true, true, true, false, false, true]};
    // let string = "109019".to_string();
    let n1: fixed_int = fixed_int::from(String::from("1234"));
    let n2: fixed_int = fixed_int::from(String::from("7"));
    let dividend: fixed_int = fixed_int::from(String::from("2.4999999999999999999"));
    let divisor: fixed_int = fixed_int{sign: false, value: vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, true, false, true, false, true, true, true, true, true, false, false, true, true, true, false, true, true, true, false, true, false, true, false, true, true, true, true, true, true, true, false, true, false, false, true, false, false, true, false, true, false, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true, true, false, false, false, true, true, true, true, true, false, false, true, false, true, false, false, true, true, false, false, false, true, true, false, true, false, true, true, true]};
    let calc = dividend.clone() / divisor.clone();
    // let f1: fixed_int = fixed_int{sign: false, value: vec![true, true, true]};
    // let f2: fixed_int = ((i8::MAX as i16 + 12) as i16).into();
    // let res = f2.parse::<i8>().unwrap();
    // println!("{:?}", fixed.bin_bcd().value);
    // println!("{:?}", fixed.bin_bcd().bcd_bin());
    // println!("{}", fixed.to_string());
    // println!("{}", fixed.bin_bcd().bcd_bin().to_string());
    // println!("{:?}", string.parse_fi());
    // println!("{:?}", string.parse_fi().to_string());
    // println!("{:?}", string.parse_bcd().to_string());
    // println!("{:?}", string.parse_bcd().to_string());
    let time = Instant::now();
    let new = n1.to_long();
    println!("conv: {:?}", time.elapsed());
    let time = Instant::now();
    new.to_bin();
    println!("reconvv: {:?}", time.elapsed());
    let new1 = n1.to_long();
    let new2 = n2.to_long();
    let second = Instant::now();
    new1.mul(new2);
    println!("{:?}", second.elapsed());
    println!("{:?}", n1.to_long().to_bin().to_string());
    println!("{:?}", (n1 / fixed_int::decimals()).to_long() << 2);
    // println!("{:?}", n1.nth_root(n2.clone()).to_string());
    functions::arithm::time_comparison(fixed_int::from(String::from("3534540035423234")), fixed_int::from(String::from("67234123421485678")));
    // println!("{}", (f1 / f2).to_string());
    // println!("{res}");
    // println!("{}", n1 < n2);
    // println!("{:?}", fi::fi1024{leading: 12, vec: vec![13, 31, 17], trailing: 123}.full_vec());
}


#[cfg(test)]
mod test;
