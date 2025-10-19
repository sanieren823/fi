mod fi;
mod conversion;

fn main() {
    let fixed = fi::fi{sign: false, value: vec![false, true, true, true, false, true, false, true]};
    println!("{:?}", fixed.bcd());
    // println!("{:?}", fi::fi1024{leading: 12, vec: vec![13, 31, 17], trailing: 123}.full_vec());
}
