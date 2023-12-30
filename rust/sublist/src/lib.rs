#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

/// Returns boolean indicating if the smaller array is found in the biggest array.
fn contains<T: PartialEq>(small: &[T], big: &[T]) -> bool {
    big.windows(small.len()).any(|window| window == small)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (_first_list.len(), _second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (a, b) if a < b => match contains(_first_list, _second_list) {
            true => Comparison::Sublist,
            false => Comparison::Unequal,
        },
        (a, b) if a > b => match contains(_second_list, _first_list) {
            true => Comparison::Superlist,
            false => Comparison::Unequal,
        },
        (_, _) => {
            if _first_list == _second_list {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
    }
}
