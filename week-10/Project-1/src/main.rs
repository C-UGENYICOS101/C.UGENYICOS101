//Define a struct for the Laptop Brand

struct LaptopBrand{
    name:String,
    unit_cost: i64,
    stock_count: i32,
}

impl LaptopBrand{
    fn calculate_purchase_cost(&self, _units_purchased: i32) -> i64 {
        self.unit_cost * (_units_purchased as i64)
    }
}

fn main() {
    const UNITS_PURCHASED: i32 = 3;

    let brands: Vec<LaptopBrand> = vec![
    LaptopBrand { name: String::from("HP"), unit_cost: 650_000, stock_count: 10},
    LaptopBrand { name: String::from("IBM"), unit_cost: 755_000, stock_count: 6},
    LaptopBrand { name: String::from("Toshiba"), unit_cost: 550_000, stock_count: 10},
    LaptopBrand { name: String::from("Dell"), unit_cost: 850_000, stock_count: 4},
    ];

   let total_cost: i64 = brands
        .iter()
        .map(|brand| brand.calculate_purchase_cost(UNITS_PURCHASED))
        .sum();

    println!("Total Cost: {}", total_cost);
}