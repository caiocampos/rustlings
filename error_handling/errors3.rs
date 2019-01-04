// errors3.rs

use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "3";

    let cost = match total_cost(pretend_user_input) {
        Ok(val) => val,
        Err(error) => panic!("Erro ao tentar calcular o custo: {:?}", error),
    };

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}