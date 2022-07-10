// errors1.rs
// This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was, instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Let's use it!
// Execute `rustlings hint errors1` for hints!

// create a kind of option by enum.Result<OK, Err>
pub fn generate_nametag_text(name: String) -> Result<String, String> {
    // use tests::{generates_nametag_text_for_a_nonempty_name, explains_why_generating_nametag_text_fails};
    if name.len() > 0 {
        // Some(format!("Hi! My name is {}", name))
        Ok(format!("Hi! My name is {}", name))
        // generates_nametag_text_for_a_nonempty_name();
    } else {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".to_string())
        // None
        // explains_why_generating_nametag_text_fails();
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    pub fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
