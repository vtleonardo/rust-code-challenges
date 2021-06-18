// Sum of Pairs: https://www.codewars.com/kata/54d81488b981293527000c8f
use std::collections::HashSet;

/// Given a list of integers and a single sum value, return the first two values
/// in order of appearance that add up to form the sum.
///
/// Time complexity O(n)
fn sum_pairs(int_list: &[i8], sum: i8) -> Option<(i8, i8)> {
    if int_list.is_empty() {
        return None;
    }
    let mut visited_set = HashSet::new();
    for int in int_list {
        let complement_sum = sum - int;
        if visited_set.contains(&complement_sum) {
            return Some((complement_sum, *int));
        }
        visited_set.insert(int);
    }
    None
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        let l1 = [1, 4, 8, 7, 3, 15];
        let l2 = [1, -2, 3, 0, -6, 1];
        let l3 = [20, -13, 40];
        let l4 = [1, 2, 3, 4, 1, 0];
        let l5 = [10, 5, 2, 3, 7, 5];
        let l6 = [4, -2, 3, 3, 4];
        let l7 = [0, 2, 0];
        let l8 = [5, 9, 13, -3];
        assert_eq!(sum_pairs(&l1, 8), Some((1, 7)));
        assert_eq!(sum_pairs(&l2, -6), Some((0, -6)));
        assert_eq!(sum_pairs(&l3, -7), None);
        assert_eq!(sum_pairs(&l4, 2), Some((1, 1)));
        assert_eq!(sum_pairs(&l5, 10), Some((3, 7)));
        assert_eq!(sum_pairs(&l6, 8), Some((4, 4)));
        assert_eq!(sum_pairs(&l7, 0), Some((0, 0)));
        assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));
    }
}
