use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (index, &value) in nums.iter().enumerate() {
        match map.get(&(target - value)) {
            Some(&prev_index) => return vec![prev_index as i32, index as i32],
            None => {
                map.insert(value, index as i32);
            }
        }
    }

    return vec![];
}

#[cfg(test)]
mod test_two_sums {
    use crate::problems::two_sums::two_sum;

    #[test]
    fn test1() {
        assert_eq!(vec![0, 1], two_sum(vec![2, 7, 11, 15], 9))
    }

    #[test]
    fn test2() {
        assert_eq!(vec![1, 2], two_sum(vec![3, 2, 4], 6))
    }

    #[test]
    fn test3() {
        assert_eq!(vec![0, 1], two_sum(vec![3, 3], 6))
    }
}