use storage::create_ingredient;
use storage::get_ingredients;

fn main() {
    create_ingredient("Cucumber".to_string());
    get_ingredients();
}
