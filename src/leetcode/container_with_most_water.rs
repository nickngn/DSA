// 11. Container With Most Water
// You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).
//
// Find two lines that together with the x-axis form a container, such that the container contains the most water.
//
// Return the maximum amount of water a container can store.
//
// Notice that you may not slant the container.

use std::cmp::min;

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut x1 = 0;
    let mut x2 = height.len() - 1;
    let mut y = 0;
    let mut area = (x2 - x1) * y;

    while x1 < x2 {
        println!("x1: {}, x2: {}, y: {}, area: {}", x1, x2, y, area);
        if min(height[x1], height[x2]) as usize * (x2 - x1) > area {
            y = min(height[x1], height[x2]) as usize;
            area = (x2 - x1) * y;
        }
        if height[x1] < height[x2] {
            x1 += 1;
        } else {
            x2 -= 1;
        }
    }
    area as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11() {
        assert_eq!(49, max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
        assert_eq!(1, max_area(vec![1, 1]));
    }

    #[test]
    fn test_12() {
        assert_eq!(4, max_area(vec![1, 2, 4, 3]));
    }
}
