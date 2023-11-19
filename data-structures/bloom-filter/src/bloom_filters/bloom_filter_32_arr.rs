use crate::bloom_filter::BloomFilter;

#[derive(Default)]
pub struct BloomFilter32 {
    bits: [bool; 32],
}

impl BloomFilter32 {
    fn additive_hasher(&self, key: &str) -> usize {
        key.chars()
            .fold(0, |acc, ch| -> usize { (acc + (ch as usize % 32)) % 32 })
    }

    fn place_value_hasher(&self, key: &str) -> usize {
        key.chars()
            .enumerate()
            .fold(0, |acc, (index, ch)| -> usize {
                let char_value = ((ch as usize) % 32) * 26usize.pow(index as u32 + 2);
                (acc + (char_value % 32)) % 32
            })
    }
}

impl std::fmt::Debug for BloomFilter32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bits: String = self
            .bits
            .iter()
            .map(|&x| if x { "1 " } else { "0 " })
            .collect();
        write!(f, "{:?}", bits)
    }
}

impl BloomFilter for BloomFilter32 {
    fn insert(&mut self, key: &str) {
        let hash_a = self.additive_hasher(key);
        let hash_b = self.place_value_hasher(key);

        self.bits[hash_a] = true;
        self.bits[hash_b] = true;
    }

    fn contains(&self, key: &str) -> bool {
        let hash_a = self.additive_hasher(key);
        let hash_b = self.place_value_hasher(key);

        self.bits[hash_a] && self.bits[hash_b]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_zeros() {
        let bl = BloomFilter32::default();

        bl.bits.iter().for_each(|bit| assert!(!bit));
    }

    #[test]
    fn test_addititve_hasher_empty_string() {
        let bl = BloomFilter32::default();
        assert_eq!(bl.additive_hasher(""), 0);
    }
}
