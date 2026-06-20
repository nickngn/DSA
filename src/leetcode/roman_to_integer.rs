
struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut s1 = s.as_str();
        let m: Vec<(&str, i32)> = vec!(
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        );
        let mut val = 0;
        while !s1.is_empty() {
            for (k, v) in m.iter() {
                if s1.starts_with(k) {
                    val += v;
                    s1 = s1.split_at(k.len()).1;
                    println!("{} {} {}", k, v, val);
                }
            }
        }
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }

}