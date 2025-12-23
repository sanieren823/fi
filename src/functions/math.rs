use crate::fi::FiBin;
use std::time::Instant;
use crate::errors::FiError;
use crate::errors::FiErrorKind;

impl FiBin {
    pub fn pow(self, n: Self) -> Self {
        if n.is_integer() {
            gen_int_pow(self, n.clone() / FiBin::decimals())
        } else {
            binominal_series(self - 1.into(), n)
        }
        
    }

    pub fn sqrt(self) -> Self {
        FiBin::new()
    }
    pub fn nth_root(self, n: Self) -> Self {
        let mut prev: FiBin = FiBin::new();
        let mut guess: FiBin = self.clone() / n.clone();
        let cur = Instant::now();
        while guess != prev {
            let single = Instant::now();
            prev = guess.clone();
            guess = newtons_method(prev.clone(), self.clone(), n.clone());
            println!("{:?}", single.elapsed());
        }
        println!("{:?}", cur.elapsed());
        guess
    }
    pub fn factorial(self) -> Self {
        FiBin::new()
    }
    pub fn termial(self) -> Self {
        let mut val = self.clone() % 1.into();
        let mut counter = self.clone() / FiBin::decimals();
        while !counter.is_zero() {
            val += counter.clone() * FiBin::decimals();
            if self.sign {
                counter += FiBin{sign: false, value: vec![true]};
            } else {
                counter -= FiBin{sign: false, value: vec![true]};
            }
        }
        val
    }
    pub fn ln(self) -> Self {
        FiBin::new()
    }

    pub fn log(self, base: Self) -> Self {
        FiBin::new()
    }

    pub fn log_10(self) -> FiBin {
        self.ln() / FiBin::from(10)
    }

    pub fn log_2(self) -> FiBin {
        self.ln() / FiBin::from(2)
    }
}


fn gen_int_pow(base: FiBin, exponent: FiBin) -> FiBin {
    let mut res: FiBin = 1.into();
    let mut counter = exponent.clone(); 
    let time = Instant::now();
    while !counter.is_zero() {
        let single = Instant::now();
        res *= base.clone();
        println!("{:?}", res.to_string());
        println!("{:?}", single.elapsed());
        if counter.sign {
            counter += FiBin{sign: false, value: vec![true]};
        } else {
            counter -= FiBin{sign: false, value: vec![true]};
        }
    }
    if exponent.sign {
        res = FiBin::from(1) / res;
    }
    println!("{:?}", time.elapsed());
    res
}

fn newtons_method(guess: FiBin, x: FiBin, n: FiBin) -> FiBin {
    let n_1 = n.clone() - 1.into();
    (guess.clone() * n_1.clone() + x / guess.pow(n_1)) / n
}

fn binominal_series(base: FiBin, exponent: FiBin) -> FiBin {
    let mut sum = FiBin::new();
    let mut temp: FiBin = 1.into();
    let mut k: FiBin = FiBin::new();
    let mut last: FiBin = FiBin::new();
    while temp != sum {
        last = binominal_coefficent(exponent.clone(), k.clone(), last.clone());
        temp = sum.clone();
        sum += base.clone().pow(k.clone()) * last.clone();

        k += 1.into();
        println!("{}", sum.to_string());
    }
    sum
}


fn binominal_coefficent_recursive(alpha: FiBin, k: FiBin) -> FiBin {
    if k.is_zero() {
        1.into()
    } else {
        let prev: FiBin = k.clone() - 1.into();
        binominal_coefficent_recursive(alpha.clone(), prev.clone()) * (alpha - prev) / k
    }
    
}

fn binominal_coefficent(alpha: FiBin, k: FiBin, prev: FiBin) -> FiBin {
    if k.is_zero() {
        1.into()
    } else {
        prev * ((alpha - (k.clone() - 1.into())) / k)
    }
    
}


fn fl_log_2(num: FiBin) -> Result<FiBin, FiError> { 
    let mut shifted;
    let mut res = FiBin::new();
    if num.sign {
        return Err(FiError::new(FiErrorKind::NumberCannotBeNegative, "Mind that the logarithmic function doesn't implement negative numbers."));
    } else if num < FiBin::one() {
        shifted = FiBin::one() / num;
        println!("num: {:?}", shifted.to_string());
        res.sign = true;  
    } else {
        shifted = num
    }
    shifted = shifted / FiBin::decimals();
    println!("{:?}", shifted.to_string());
    
    while shifted.len() > 1 {
        shifted = shifted.clone() >> 1;
        if res.sign {
            res -= 1.into();
        } else {
            res += 1.into();
        }
    }
    Ok(res)
}