/** 
 * Min Sub Array Length:
 * Write a function called min_sub_array_len which accepts an array
 * of positive integers and a positive integer.
 *
 * This function should return the minimal length of a contiguous subarray
 * of which the sum is greater than or equal to the integer passed to the 
 * function. If there isn't one, return 0.
*/

fn min_sub_array_len(list: Vec<u32>, min: u32) -> usize {
        let last = list.len()-1;
        let mut pl = 0;
        let mut pr = 0;
        let mut current = list[0];
        let mut smallest = list.len()+1;
        let mut wsize = 1;

        while pr <= last {
            if current < min {
                pr += 1;
                wsize += 1;
                if pr <= last { current += list[pr] };
            } else {
                if wsize < smallest { smallest = wsize }
                current -= list[pl];
                pl += 1;
                wsize -= 1;
            }
        }
        
        if smallest <= list.len() { smallest } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(min_sub_array_len(vec![2, 3, 1, 2, 4, 3], 7), 2);
    }

    #[test]
    fn test_1() {
        assert_eq!(min_sub_array_len(vec![2, 1, 6, 5, 4], 9), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(min_sub_array_len(vec![3, 1, 7, 11, 2, 9, 8, 21, 62, 33, 19], 52), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(min_sub_array_len(vec![1, 4, 16, 22, 5, 7, 8, 9, 10], 39), 3);
    }

    #[test]
    fn test_4() {
        assert_eq!(min_sub_array_len(vec![1, 4, 16, 22, 5, 7, 8, 9, 10], 55), 5);
    }

    #[test]
    fn test_5() {
        assert_eq!(min_sub_array_len(vec![4, 3, 3, 8, 1, 2, 3], 11), 2);
    }

    #[test]
    fn test_6() {
        assert_eq!(min_sub_array_len(vec![1, 4, 16, 22, 5, 7, 8, 9, 10], 95), 0);
    }
}
