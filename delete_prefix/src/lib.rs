pub fn delete_prefix<'a, 'b>(prefix: &'b str, s: &'a str) -> Option<&'a str> {
    if prefix.len() > s.len() {
        return None;
    }
    let mut char_s = s.chars();

    for char_prefix in prefix.chars() {
        if char_prefix != char_s.next()? {
            return None;
        }
    }
    Some(&s[prefix.len()..])
}
