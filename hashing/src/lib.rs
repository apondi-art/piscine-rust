use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted_list = list.to_vec();
    sorted_list.sort();
    let len = sorted_list.len();
    if len % 2 == 1 {
        sorted_list[len / 2]
    } else {
        let mid1 = sorted_list[(len / 2) - 1];
        let mid2 = sorted_list[len / 2];
        (mid1 + mid2) / 2
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut frequency_map = HashMap::new();
    for &num in list {
        *frequency_map.entry(num).or_insert(0) += 1;
    }
    frequency_map
        .into_iter()
        .max_by_key(|&(num, count)| (count, -num))
        .map(|(num, _)| num)
        .unwrap()
}
