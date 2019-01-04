/**
 * Sorted Frequency:
 * Given a sorted array and a number, write a function called sorted_frequency
 * that counts the occurances of the number in the array
 */
use math::round;

fn sorted_frequency(list: Vec<i32>, n: i32) -> i32 {
    let length = list.len();

    let mut l = 0;
    let mut r = length - 1;

    let mut lidx: i32 = 1;
    // find left bound
    if list[0] == n {
        lidx = 0;
    } else {
        while l <= r {
            let mid = round::floor((l as f64 + r as f64) / 2.0, 0) as usize;
            if list[mid] < n {
                l = mid + 1;
            } else if list[mid] > n {
                r = mid - 1;
            } else {
                if list[mid - 1] < n {
                    lidx = mid as i32;
                    break;
                } else {
                    r = mid - 1;
                }
            }
        }
    }

    l = lidx as usize;
    r = length - 1;
    let mut ridx: i32 = -1;
    // find right bound
    if list[length - 1] == n {
        ridx = length as i32 - 1;
    } else {
        while l <= r {
            let mid = round::floor((l as f64 + r as f64) / 2.0, 0) as usize;
            if list[mid] < n {
                l = mid + 1;
            } else if list[mid] > n {
                r = mid - 1;
            } else {
                if list[mid + 1] > n {
                    ridx = mid as i32;
                    break;
                } else {
                    l = mid + 1;
                }
            }
        }
    }

    ridx - lidx + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(sorted_frequency(vec![1, 1, 2, 2, 2, 2, 3], 2), 4);
    }

    #[test]
    fn test_1() {
        assert_eq!(sorted_frequency(vec![1, 1, 2, 2, 2, 2, 3], 3), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(sorted_frequency(vec![1, 1, 2, 2, 2, 2, 3], 1), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(sorted_frequency(vec![1, 1, 2, 2, 2, 2, 3], 4), -1);
    }
}
