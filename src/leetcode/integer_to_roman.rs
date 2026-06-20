struct IntegerToRomanSolution {}

impl IntegerToRomanSolution {

    pub fn int_to_roman(num: i32) -> String {
        let mut i = 1000;
        let mut r_num = String::from("");
        let mut num2 = num;
        if num2 > i {
            r_num.push_str("M".repeat((num2 / i) as usize).as_str());
            num2 = num2 % i;
        } else if num2 == i {
            r_num.push_str("M");
            return r_num;
        }

        i = 100;
        if num2 > i {
            let mut m = num2 / i;
            if m == 4 {
                r_num.push_str("CD");
            } else if m == 9 {
                r_num.push_str("CM");
            } else if m < 4 {
                r_num.push_str("I".repeat(m as usize).as_str());
            } else {
                r_num.push_str("D");
                r_num.push_str("C".repeat((m - 5) as usize).as_str());
            }
            num2 = num2 % i;
        } else if num2 == i {
            r_num.push_str("C");
            return r_num;
        }

        i = 10;
        if num2 > i {
            let mut m = num2 / i;
            if m == 4 {
                r_num.push_str("XL");
            } else if m == 9 {
                r_num.push_str("XC");
            } else if m < 4 {
                r_num.push_str("X".repeat(m as usize).as_str());
            } else {
                r_num.push_str("L");
                r_num.push_str("X".repeat((m - 5) as usize).as_str());
            }
            num2 = num2 % i;
        } else if num2 == i {
            r_num.push_str("X");
            return r_num;
        }

        i = 0;
        if num2 > i {
            let mut m = num2;
            if m == 4 {
                r_num.push_str("IV");
            } else if m == 9 {
                r_num.push_str("IX");
            } else if m < 4 {
                r_num.push_str("I".repeat(m as usize).as_str());
            } else {
                r_num.push_str("V");
                r_num.push_str("I".repeat((m - 5) as usize).as_str());
            }
        }

        println!("{}", r_num.clone().as_str());
        r_num
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::integer_to_roman::IntegerToRomanSolution;

    #[test]
    fn test1() {
        assert_eq!(IntegerToRomanSolution::int_to_roman(3749), "MMMDCCXLIX");
    }
    #[test]
    fn test2() {
        assert_eq!(IntegerToRomanSolution::int_to_roman(58), "LVIII");
    }
    #[test]
    fn test3() {
        assert_eq!(IntegerToRomanSolution::int_to_roman(1994), "MCMXCIV");
    }
    #[test]
    fn test4() {
        assert_eq!(IntegerToRomanSolution::int_to_roman(1), "I");
        assert_eq!(IntegerToRomanSolution::int_to_roman(10), "X");
        assert_eq!(IntegerToRomanSolution::int_to_roman(11), "XI");
    }
}