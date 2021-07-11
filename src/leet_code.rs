use std::vec::Vec;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![-1, -1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2i32, 7, 11, 15], 9), vec![0i32, 1]);
        assert_eq!(two_sum(vec![3i32, 2, 4], 6), vec![1i32, 2]);
        assert_eq!(two_sum(vec![3i32, 3], 6), vec![0i32, 1]);
    }
}
