use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};

/// A container for pairs where order shouldn't be considered
pub struct Pair<T> {
    a: T,
    b: T,
}

impl<T: Eq + Hash> Pair<T> {
    pub fn new(a: T, b: T) -> Pair<T> {
        Pair { a, b }
    }
}

impl<T: Eq + Hash> PartialEq for Pair<T> {
    fn eq(&self, rhs: &Self) -> bool {
        (self.a == rhs.a && self.b == rhs.b) || (self.a == rhs.b && self.b == rhs.b)
    }
}

impl<T: Eq + Hash> Eq for Pair<T> {}

impl<T: Eq + Hash> Hash for Pair<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        let state = RandomState::new();
        let mut a = state.build_hasher();
        let mut b = state.build_hasher();
        self.a.hash(&mut a);
        self.b.hash(&mut b);
        hasher.write_u64(a.finish() ^ b.finish());
    }
}
