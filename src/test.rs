use crate::fi::fi;
use super::*;

#[test]
fn create_new() {
    assert_eq!(fi::new(), fi{sign: false, value: Vec::new()});
}


#[test]
fn reg_add() {
    let num1: i8 = 7;
    let num2: i8 = 9;
    let res: i8 = 7 + 9;
    let fixed_int: fi = res.into();
    println!("{:?}", fixed_int);
    assert_eq!(fixed_int, num1.to_string().parse_fi() + num2.to_string().parse_fi()); // adjsut after implementing string conversion
}