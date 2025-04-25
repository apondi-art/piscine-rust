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
        // Sort items by price for discount calculation
        let mut items_sorted: Vec<&(String, f32)> = self.items.iter().collect();
        items_sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        
        // Calculate discount
        let total_items = items_sorted.len();
        let free_items = total_items / 3;
        
        // Calculate raw adjusted prices with exact same discount ratio for all items
        let mut prices: Vec<f32> = self.items.iter().map(|(_, p)| *p).collect();
        
        if free_items > 0 {
            // Calculate total price before discount
            let total_before: f32 = prices.iter().sum();
            
            // Calculate discount amount from free items
            let discount_amount: f32 = items_sorted.iter().take(free_items).map(|(_, p)| p).sum();
            
            // Calculate discount ratio
            let discount_ratio = discount_amount / total_before;
            
            // Apply discount to all items
            for price in &mut prices {
                *price *= (1.0 - discount_ratio);
                // Round to 2 decimal places
                *price = (*price * 100.0).round() / 100.0;
            }
        }
        
        // Create receipt with sorted prices
        let mut receipt = prices.clone();
        receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        self.receipt = receipt.clone();
        receipt
    }
}