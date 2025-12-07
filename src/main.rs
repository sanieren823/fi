mod fi;
mod conversion;
// use crate::fi::Parsing;
mod functions;
use crate::fi::fi as ll;

fn main() {
    // let fixed = fi::fi{sign: false, value: vec![false, true, false, true, true, true, false, false, true]};
    // let string = "109019".to_string();
    let n1: ll = (43 as i8).into();
    let n2: ll = (5 as i8).into();
    let calc = n1.clone() % n2.clone();
    // println!("{:?}", fixed.bin_bcd().value);
    // println!("{:?}", fixed.bin_bcd().bcd_bin());
    // println!("{}", fixed.to_string());
    // println!("{}", fixed.bin_bcd().bcd_bin().to_string());
    // println!("{:?}", string.parse_fi());
    // println!("{:?}", string.parse_fi().to_string());
    // println!("{:?}", string.parse_bcd().to_string());
    // println!("{:?}", string.parse_bcd().to_string());
    println!("{:?}", n1.to_string());
    println!("{:?}", n2.to_string());
    println!("{:?}", calc.to_string());
    println!("{}", n1 < n2);
    // println!("{:?}", fi::fi1024{leading: 12, vec: vec![13, 31, 17], trailing: 123}.full_vec());
}


#[cfg(test)]
mod test;
