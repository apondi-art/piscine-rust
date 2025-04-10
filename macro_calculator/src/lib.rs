use json;

#[derive(Debug)]
pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut cals_sum = 0.0;
    let mut carbs_sum = 0.0;
    let mut proteins_sum = 0.0;
    let mut fats_sum = 0.0;

    for food in foods {
        // Parse the kcal value from the second element of the calories array
        let kcal_str = &food.calories[1];
        let kcal_value: f64 = kcal_str
            .trim_end_matches("kcal")
            .parse()
            .expect("Invalid kcal format");
        cals_sum += kcal_value * food.nbr_of_portions;

        // Calculate each macro's total considering portions
        proteins_sum += food.proteins * food.nbr_of_portions;
        fats_sum += food.fats * food.nbr_of_portions;
        carbs_sum += food.carbs * food.nbr_of_portions;
    }

    // Function to round to two decimal places
    fn round_to_two_decimal(value: f64) -> f64 {
        (value * 100.0).round() / 100.0
    }

    // Round each sum
    cals_sum = round_to_two_decimal(cals_sum);
    proteins_sum = round_to_two_decimal(proteins_sum);
    fats_sum = round_to_two_decimal(fats_sum);
    carbs_sum = round_to_two_decimal(carbs_sum);

    // Create the JSON object
    json::object! {
        "cals" => cals_sum,
        "carbs" => carbs_sum,
        "proteins" => proteins_sum,
        "fats" => fats_sum,
    }
}


