/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let (digit_chars, other_chars): (Vec<char>, Vec<char>) =
        code.chars().partition(|c| *c >= '0' && *c <= '9');

    if digit_chars.len() <= 1 || other_chars.iter().any(|c| *c != ' ') {
        return false;
    }

    digit_chars
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, c)| {
            if idx % 2 != 0 {
                let mut number = c.to_digit(10).unwrap_or(0) * 2;
                if number > 9 {
                    number -= 9
                }
                number
            } else {
                c.to_digit(10).unwrap_or(0)
            }
        })
        .reduce(|acc, e| acc + e)
        .unwrap()
        % 10
        == 0
}
