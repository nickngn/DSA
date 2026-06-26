use std::collections::HashMap;

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut c: Vec<i32> = nums.clone();
    c.sort();

    if c.is_empty() || c[0] > 0 {
        return vec![];
    }
    if c.len() > 2 && (c[0] == c[1] && c[1] == c[2] && c[0] == 0) {
        return vec![vec![0, 0, 0]];
    }
    let mut m: HashMap<i32, HashMap<i32, i32>> = HashMap::new();

    let mut keys: HashMap<i32, usize> = HashMap::new();
    for i1 in 0..c.len() {
        keys.insert(c[i1], i1);
    }

    for i1 in 0..c.len() {
        for i2 in (i1 + 1)..c.len() {
            let v3 = -(c[i1] + c[i2]);

            if !keys.get(&v3).is_some_and(|i| i > &i2) || v3 < c[i2] {
                continue;
            }
            if m.get(&c[i1]).is_some_and(|h| h.contains_key(&c[i2])) {
                continue;
            }
            if m.get_mut(&(c[i1])).is_none() {
                m.insert(c[i1], HashMap::from([(c[i2], v3)]));
            } else {
                m.get_mut(&(c[i1])).unwrap().insert(c[i2], v3);
            }
        }
    }

    let mut result: Vec<Vec<i32>> = vec![];
    m.iter().for_each(|entry| {
        entry
            .1
            .iter()
            .for_each(|c_entry| result.push(vec![*entry.0, *c_entry.0, *c_entry.1]))
    });

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    const EMPTY: Vec<Vec<i32>> = vec![];

    #[test]
    pub fn test1() {
        assert_eq!(
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            three_sum(vec![-1, 0, 1, 2, -1, -4])
        );
    }

    #[test]
    pub fn test2() {
        assert_eq!(EMPTY, three_sum(vec![0, 1, 1]));
    }

    #[test]
    pub fn test3() {
        assert_eq!(vec![vec![0, 0, 0]], three_sum(vec![0, 0, 0]));
    }

    #[test]
    pub fn test4() {
        assert_eq!(
            vec![vec![-100, -60, 160], vec![-70, -60, 130]],
            three_sum(vec![-100, -70, -60, 110, 120, 130, 160])
        );
    }
}
