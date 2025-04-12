pub fn get_diamond(c: char) -> Vec<String> {
    if c == 'A' {
        return vec!["A".to_string()];
    }

    let size = (c as u8 - b'A' + 1) as usize;
    let total_lines = 2 * size - 1;
    let mut diamond = Vec::with_capacity(total_lines);

    // Top half including the middle
    for i in 0..size {
        let current_char = (b'A' + i as u8) as char;
        let leading_spaces = size - 1 - i;
        let line = if current_char == 'A' {
            format!(
                "{}{}{}",
                " ".repeat(leading_spaces),
                'A',
                " ".repeat(leading_spaces)
            )
        } else {
            let middle_spaces = 2 * i - 1;
            format!(
                "{}{}{}{}{}",
                " ".repeat(leading_spaces),
                current_char,
                " ".repeat(middle_spaces),
                current_char,
                " ".repeat(leading_spaces)
            )
        };
        diamond.push(line);
    }

    // Bottom half (mirror of top without the middle)
    for i in (0..size - 1).rev() {
        diamond.push(diamond[i].clone());
    }

    diamond
}