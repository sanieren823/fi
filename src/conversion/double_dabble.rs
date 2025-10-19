
use crate::fi::fi;

struct u4([bool; 4]);



// #[inline(always)]
// fn ceil_usize(input: f64) -> usize {
//     input.ceil() as usize
// }

// #[inline(always)]
// fn floor_usize(input: f64) -> usize {
//     input.floor() as usize
// } 
#[inline(always)]
fn to_dec(input: &[bool]) -> usize {
    let mut reversed: [bool; 4] = input.try_into().unwrap_or([false; 4]);
    reversed.reverse();
    match reversed {
        [false, false, false, false] => 0,
        [false, false, false, true]  => 1,
        [false, false, true, false]  => 2,
        [false, false, true, true]   => 3,
        [false, true, false, false]  => 4,
        [false, true, false, true]   => 5,
        [false, true, true, false]   => 6,
        [false, true, true, true]    => 7,
        [true,  false, false, false] => 8,
        [true,  false, false, true]  => 9,
        [true,  false, true, false]  => 10,
        [true,  false, true, true]   => 11,
        [true,  true,  false, false] => 12,
        [true,  true,  false, true]  => 13,
        [true,  true,  true, false]  => 14,
        [true,  true,  true, true]   => 15,
        _ => 0,
    }
}


// should be more efficient for u4
#[inline(always)]
fn plus_three(input: &[bool]) -> [bool; 4] {
    let mut s = [false; 4];
    let mut reversed: [bool; 4] = input.try_into().unwrap_or([false; 4]);
    reversed.reverse();
    match reversed {
        [false, false, false, false] => s = [false, false, true, true],
        [false, false, false, true] => s = [false, true, false, false],
        [false, false, true, false] => s = [false, true, false, true],
        [false, false, true, true] => s = [false, true, true, false],
        [false, true, false, false] => s = [false, true, true, true],
        [false, true, false, true] => s = [true, false, false, false],
        [false, true, true, false] => s = [true, false, false, true],
        [false, true, true, true] => s = [true, false, true, false],
        [true, false, false, false] => s = [true, false, true, true],
        [true, false, false, true] => s = [true, true, false, false],
        [true, false, true, false] => s = [true, true, false, true],
        [true, false, true, true] => s = [true, true, true, false],
        [true, true, false, false] => s = [true, true, true, true],
        [true, true, false, true] => s = [false, false, false, false],
        [true, true, true, false] => s = [false, false, false, true],
        [true, true, true, true] => s = [false, false, true, false],
        _ => s = [false, false, true, true], // TODO: figure out what's missing
    }
    s.reverse();
    s
}



impl fi {
    pub fn bcd(&self) -> Vec<Vec<bool>> {
        let length: usize = self.value.len();
        if length <= 4 {
            vec![self.value.clone()]
        } else {
            let mut output: Vec<bool> = vec![false; length + (length - 4) / 3 + 1]; // why +1: because index != length
            println!("{}", output.len());
            for index in 0..length {
                output[index] = self.value[index];
            }
            output.iter().rev();
            for i in 0..=(length - 4) {
                for j in 0..=(i / 3) {
                    let current = length - i + j * 4;
                    println!("{}", current);
                    if to_dec(&output[current - 3..=current]) > 4 { // actually 4 going down from current (inverted because that's how slices work in rust)
                        plus_three(&output[current - 3..=current]);
                    }
                }
            }
        
            let mut vec: Vec<Vec<bool>> = Vec::new();
            let mut arr = vec![false; 4];
            let num_last = output.len() % 4;
            println!("{:?}", output);
    
            // fill the list so it's easiably matchable
            if num_last > 0 {                                                 
                arr[(4 - num_last)..4].copy_from_slice(&output[0..num_last]);
                vec.push(arr.clone());
            }
            
            for chunk in output[num_last..].chunks(4) {
                vec.push(chunk.to_vec());
            }
    
            vec
        }
        
    } 
}
  


