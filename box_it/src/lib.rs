pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let nums: Vec<u32> = s.split_whitespace()
        .filter_map(|c| {
            if c.ends_with('k') {
                c.trim_end_matches('k')
                    .parse::<u32>()
                    .ok()
                    
                    .map(|n| n * 1000)
            } else {
                c.parse::<u32>().ok()
            }
        })
        .collect();
    Box::new(nums)
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}

