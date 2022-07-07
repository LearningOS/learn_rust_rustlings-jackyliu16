// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!


use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError>{
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;
    // let cost = match pretend_user_input.parse::<i32>(){      // we couldn't using this here is because we want &str
    //     Ok(val) => total_cost(pretend_user_input),           // but ? could make it possible for we to return a error ! [if input not match]
    //     Err(e) => return Err(e),
    // };

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
