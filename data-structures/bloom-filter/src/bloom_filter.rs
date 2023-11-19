pub trait BloomFilter {
    fn insert(&mut self, key: &str);

    /// maybe yes, definitely no
    fn contains(&self, key: &str) -> bool;
}
