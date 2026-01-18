#[cfg(test)]
mod tests {
    use finum::finum::{FiLong, FiBin};
    #[test]
    fn string() {
        assert_eq!(FiLong::neg_four(), FiLong::from("-4"));
        assert_eq!(FiLong::one_quarter(), FiLong::from(String::from("0.25")));
        assert_eq!(FiLong::seven().to_string(), String::from("7.00000000000000000000"));
        assert_eq!(FiLong::neg_eight().to_string(), String::from("-8.00000000000000000000"));
    }

    #[test]
    fn int() {
        assert_eq!(FiLong::neg_two(), FiLong::from(-2));
        assert_eq!(FiLong::nine(), FiLong::from(9u32)); // i'm not testing every type since there implemented exactly the same
        assert_eq!(FiLong::three().parse::<u8>().unwrap(), 3u8);
        assert_eq!(FiLong::neg_ten().parse::<i128>().unwrap(), -10i128);
    }

    #[test]
    fn float() {
        assert_eq!(FiLong::one_fifth(), FiLong::from(0.2f32));
        assert_eq!(FiLong::neg_five(), FiLong::from(-5.0f64)); 
        assert_eq!(FiLong::one_half().parse_f32().unwrap(), 0.5f32);
        assert_eq!(FiLong::neg_one().parse_f64().unwrap(), -1.0f64);
    }

    #[test]
    fn internal() {
        assert_eq!(FiBin::one().to_long(), FiLong::one());
        assert_eq!(FiBin::one(), FiLong::one().to_bin());
        assert_eq!(FiLong::e(), FiLong::e().to_bin().to_long());
    }
}