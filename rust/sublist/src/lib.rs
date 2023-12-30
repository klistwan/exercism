#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

/// Returns boolean indicating if l1 has the subesquence l2.
fn contains<T: PartialEq>(small: &[T], big: &[T]) -> bool {
    big.windows(small.len()).any(|window| window == small)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    } else if _first_list.is_empty() {
        return Comparison::Sublist;
    } else if _second_list.is_empty() {
        return Comparison::Superlist;
    } else if _first_list.len() < _second_list.len() && contains(_first_list, _second_list) {
        return Comparison::Sublist;
    } else if _second_list.len() < _first_list.len() && contains(_second_list, _first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}
