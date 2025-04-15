pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let nums: Vec<u32> = s.split_whitespace()
        .map(|c| {
            if c.ends_with('k') {
                // Convert "5.5k" to 5500, for example
                if let Some(num_str) = c.strip_suffix('k') {
                    if num_str.contains('.') {
                        // Handle decimal values like "5.5k"
                        if let Ok(num) = num_str.parse::<f32>() {
                            return (num * 1000.0) as u32;
                        }
                    } else {
                        // Handle integer values like "5k"
                        if let Ok(num) = num_str.parse::<u32>() {
                            return num * 1000;
                        }
                    }
                }
                0 // Default if parsing fails
            } else {
                c.parse::<u32>().unwrap_or(0)
            }
        })
        .collect();
    Box::new(nums)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}