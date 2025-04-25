use std::collections::HashMap;
use std::hash::Hash;

pub fn slices_to_map<'a, T, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U>
where
    T: Hash + Eq,
{
    let min_len = std::cmp::min(keys.len(), values.len());
    
    keys.iter()
        .zip(values.iter())
        .take(min_len)
        .map(|(k, v)| (k, v))
        .collect()
}