use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();

    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut nums = list.to_vec();
    nums.sort();

    let mid = nums.len() / 2;

    if nums.len() % 2 == 0 {
        (nums[mid - 1] + nums[mid]) / 2
    } else {
        nums[mid]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut counts = HashMap::new();

    for n in list {
        *counts.entry(*n).or_insert(0) += 1;
    }

    let mut best_num = list[0];
    let mut best_count = 0;

    for (num, count) in counts {
        if count > best_count {
            best_num = num;
            best_count = count;
        }
    }

    best_num
}