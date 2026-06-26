pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 1 {
        return strs[0].clone();
    }
    if strs[0].len() == 0 {
        return String::from("");
    }
    let mut common = String::from("");

    let mut len = 1;
    while len <= strs[0].len() {
        let test = strs[0].split_at(len).0;
        for s in &strs {
            if !s.starts_with(test) {
                return common;
            }
        }
        common = test.to_string();
        len = len + 1;
    }

    common
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test1() {
        assert_eq!(
            String::from("fl"),
            longest_common_prefix(vec!(
                "flower".to_owned(),
                "flow".to_owned(),
                "flight".to_owned()
            ))
        );
    }

    #[test]
    pub fn test2() {
        assert_eq!(
            String::from(""),
            longest_common_prefix(vec!(
                "dog".to_owned(),
                "racecar".to_owned(),
                "car".to_owned()
            ))
        );
    }

    #[test]
    pub fn test3() {
        assert_eq!(
            String::from("a"),
            longest_common_prefix(vec!("a".to_owned()))
        );
    }

    #[test]
    pub fn test4() {
        assert_eq!(String::from(""), longest_common_prefix(vec!("".to_owned())));
    }

    #[test]
    pub fn test5() {
        assert_eq!(
            String::from("flower"),
            longest_common_prefix(vec!(
                "flower".to_owned(),
                "flower".to_owned(),
                "flower".to_owned(),
                "flower".to_owned(),
                "flower".to_owned()
            ))
        );
    }
}
