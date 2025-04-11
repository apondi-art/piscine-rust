pub fn talking(s: &str) -> &'static str {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return "Just say something!";
    }

    let is_yell = {
        let has_letters = trimmed.chars().any(|c| c.is_alphabetic());
        has_letters && trimmed.chars().all(|c| !c.is_alphabetic() || c.is_uppercase())
    };

    if trimmed.ends_with('?') {
        if is_yell {
            "Quiet, I am thinking!"
        } else {
            "Sure."
        }
    } else if is_yell {
        "There is no need to yell, calm down!"
    } else {
        "Interesting"
    }
}
