// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references


fn main() {
    let mut data = "Rust is great!".to_string();    // String 

    get_char(&data);                                // just give a kind of borrow to them 

    string_uppercase(&mut data);                    // giving a kind of mutiable reference
}

// Should not take ownership
fn get_char(data: &String) -> char {                // a kind of borrow
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &mut String) {            // mutiable reference 
    
    let data = &data.to_uppercase();

    println!("{}", data);
}
