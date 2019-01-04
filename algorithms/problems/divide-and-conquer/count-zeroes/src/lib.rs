/**
 * Count Zeroes:
 * Given an array of 1s and 0s which has all 1s first followed by all 0s, write a
 * function called count_zeroes which returns the number of zeroes in the array
 */

use math::round;

fn count_zeroes(list: Vec<i32>) -> usize {
    let length = list.len();

    // handle edge cases all zeroes and no zeroes
    if list[0] == 0 { return length };
    if list[length-1] != 0 { return 0 };

    let mut l = 0;
    let mut r = length-1;

    // because we fail early in cases where we get a sorted
    // list with no zeroes this should be a loop not a while
    // because there's no need for a conditional check -
    // we are garaunteed to find the breakpoint
    loop {
        let mid = round::floor((l as f64 + r as f64)/2.0, 0) as usize;
        if list[mid] > 0 {
            l = mid + 1;
        } else if list[mid] == 0 {
            if list[mid-1] == 1 {
                return length - mid;
            } else {
                r = mid - 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(count_zeroes(vec![1, 1, 1, 1, 0, 0]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(count_zeroes(vec![1, 0, 0, 0, 0]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(count_zeroes(vec![0, 0, 0]), 3);
    }

    #[test]
    fn test_4() {
        assert_eq!(count_zeroes(vec![1, 1, 1, 1]), 0);
    }
}
