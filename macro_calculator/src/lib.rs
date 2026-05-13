pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs : f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

fn round_two(n: f64) -> f64 {
    (n * 100.0).round() / 100.0
}

fn kcal_to_number(kcal: &str) -> f64 {
    kcal.replace("kcal", "").parse::<f64>().unwrap()
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;

    for food in foods {
        cals += kcal_to_number(&food.calories.1) * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
    }

    json::object! {
        "cals" => round_two(cals),
        "carbs" => round_two(carbs),
        "proteins" => round_two(proteins),
        "fats" => round_two(fats),
    }
}