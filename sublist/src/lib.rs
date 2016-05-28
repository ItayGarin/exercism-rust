#[derive(PartialEq, Eq, Debug)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
if a == b {
    Comparison::Equal
} else if contains(a, b) {
    Comparison::Superlist
} else if contains(b, a) {
    Comparison::Sublist
} else {
    Comparison::Unequal
}
}

fn contains<T: PartialEq>(collection: &[T], sublist: &[T]) -> bool {
    for i in 0 .. collection.len() {
        let slice = &collection[i..];
        if slice.len() < sublist.len() {
            return false;
        }
        if slice.starts_with(sublist) {
            return true;
        }
    }

    return false;
}
