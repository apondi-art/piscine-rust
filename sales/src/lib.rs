#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some(product) = s.products.iter().find(|(name, _)| name == &ele) {
            self.items.push(product.clone());
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut items_sorted = self.items.clone();
        items_sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    
        let total_items = items_sorted.len();
        let free_items = total_items / 3;
        let total_discount: f32 = items_sorted.iter().take(free_items).map(|(_, price)| price).sum();
    
        let total_original: f32 = items_sorted.iter().map(|(_, price)| price).sum();
        let total_after_discount = total_original - total_discount;
    
        let mut adjusted_prices = Vec::new();
        for (_, price) in &self.items {
            let ratio = price / total_original;
            let discounted_price = price - (ratio * total_discount);
            // Use floor instead of round for more predictable results
            let rounded_price = (discounted_price * 100.0).floor() / 100.0;
            adjusted_prices.push(rounded_price);
        }
    
        // Handle any remaining discrepancy
        let sum_adjusted: f32 = adjusted_prices.iter().sum();
        let discrepancy = total_after_discount - sum_adjusted;
        if discrepancy.abs() > 0.001 {
            let last_index = adjusted_prices.len() - 1;
            adjusted_prices[last_index] += discrepancy;
            adjusted_prices[last_index] = (adjusted_prices[last_index] * 100.0).floor() / 100.0;
        }
    
        adjusted_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.receipt = adjusted_prices.clone();
        adjusted_prices
    }
}