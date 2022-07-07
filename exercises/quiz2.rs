// quiz2.rs
// This is a quiz for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned()); // using a kind of clone to create a owned data from borrowed data
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));                 // format create a kind of String
    string_slice(&String::from("abc")[0..1]);                       // a kind of &str
    string_slice("  hello there ".trim());                          // delete the blank at head and tail
    string("Happy Monday!".to_string().replace("Mon", "Tues"));     // replace all things in string
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());                // after change it will become string
}
