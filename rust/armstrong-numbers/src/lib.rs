/// Returns boolean indicating if number is an Armstrong number.
/// An Armstrong number is a number that is the sum of its own digits,
/// each raised to the power of the number of digits.
pub fn is_armstrong_number(num: u64) -> bool {
    let number_of_digits = num.to_string().len() as u32;
    num == num.to_string().chars().fold(0, |acc, e| {
        acc + (e.to_digit(10).unwrap() as u64).pow(number_of_digits)
    })
}
